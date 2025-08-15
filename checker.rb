#!/usr/bin/env ruby
# frozen_string_literal: true

SAGUARO = './target/release/saguaro_cli'
ORACLE = 'z3'

SAT = 's SATISFIABLE'
UNSAT = 's UNSATISFIABLE'

def check(filename)
  saguaro_ans = `#{SAGUARO} #{filename}`
  oracle_ans = `#{ORACLE} #{filename}`

  # TODO: Check that saguaro's satisfying assignments are valid
  (saguaro_ans.include?(SAT) && oracle_ans.include?(SAT)) ||
    (saguaro_ans.include?(UNSAT) && oracle_ans.include?(UNSAT))
end

def main
  if ARGV.empty?
    puts 'usage: ./checker.rb <cnf file>'
    exit
  end

  puts(check(ARGV[0]) ? 'PASS' : 'FAIL')
end

main if __FILE__ == $PROGRAM_NAME
