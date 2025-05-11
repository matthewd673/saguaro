#!/usr/bin/env ruby
# frozen_string_literal: true

SAGUARO = './target/release/saguaro'
Z3 = 'z3'

SAT = 's SATISFIABLE'
UNSAT = 's UNSATISFIABLE'

def check(filename)
  saguaro_ans = `#{SAGUARO} #{filename}`
  z3_ans = `#{Z3} #{filename}`

  # TODO: Check that saguaro's satisfying assignments are valid
  (saguaro_ans.include?(SAT) && z3_ans.include?(SAT)) ||
    (saguaro_ans.include?(UNSAT) && z3_ans.include?(UNSAT))
end

def main
  if ARGV.empty?
    puts 'usage: ./checker.rb <cnf file>'
    exit
  end

  puts(check(ARGV[0]) ? 'PASS' : 'FAIL')
end

main if __FILE__ == $PROGRAM_NAME
