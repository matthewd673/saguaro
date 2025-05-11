#!/usr/bin/env ruby
# frozen_string_literal: true

require 'timeout'

CHECKER_ID = "<CHECKER>"

SOLVERS = {
  'saguaro' => './target/release/saguaro $f',
  'z3' => 'z3 $f',
  'minisat' => 'minisat $f',
  CHECKER_ID => './checker.rb $f'
}.freeze

DEFAULT_TIMEOUT = 10

if ARGV.empty?
  puts 'usage: ./benchmark.rb <directory> [timeout secs]'
  exit
end

def build_saguaro
  puts '=== BUILDING ==='
  `cargo build --release`
end

def run_solvers(problem_dir, timeout)
  puts '=== RUNNING ==='

  cnf_files = Dir.entries(problem_dir.to_s).filter! do |e|
    !File.directory?(e) && e.end_with?('.cnf')
  end

  stats = {}
  SOLVERS.each_key do |s|
    stats[s] = []
  end

  cnf_files.each do |f|
    puts "#{f}..."
    SOLVERS.each do |s, c|
      puts "  #{s}..."
      start_time = Time.now

      begin
        cmd = c.gsub('$f', "#{problem_dir}/#{f}")
        pipe = IO.popen(cmd, 'r')

        Timeout.timeout(timeout) do
          Process.wait(pipe.pid)
        end
      rescue Timeout::Error
        Process.kill(9, pipe.pid)
        stats[s].push({ file: f, result: "TIMEOUT (>#{timeout}s)" })
        next
      end

      end_time = Time.now

      if $CHILD_STATUS && $CHILD_STATUS.exitstatus != 0
        stats[s].push({ file: f, result: 'CRASH' })
      elsif s != CHECKER_ID
        stats[s].push({ file: f, secs: end_time - start_time })
      else
        stats[s].push({ file: f, result: pipe.gets(nil) })
      end

      pipe.close
    end
  end

  return stats
end

def print_stats(stats)
  puts '=== RESULTS ==='

  SOLVERS.each_key do |s|
    puts "#{s} =>\n"
    stats[s].each do |r|
      if r[:secs]
        result = "#{r[:secs]}s"
      elsif r[:result].strip! == 'PASS' # Special handling for checker "PASS"
        result = "\e[32m#{r[:result]}\e[0m"
      else # All other results are assumed to be errors (e.g. crash, timeout)
        result = "\e[31m#{r[:result]}\e[0m"
      end

      puts "  #{r[:file]}\t\t#{result}"
    end
  end
end

def main
  problem_dir = ARGV[0]
  timeout = ARGV.length > 1 ? ARGV[1].to_i : DEFAULT_TIMEOUT

  # Build a fresh version of saguaro before running
  build_saguaro

  stats = run_solvers(problem_dir, timeout)
  print_stats(stats)
end

main if __FILE__ == $PROGRAM_NAME
