Describe 'rjo'
  It 'make object'
    When call rjo name=gorilla likes='["a","b","c"]'
    The output should equal '{"likes":["a","b","c"],"name":"gorilla"}'
  End

  It 'make array'
    When call rjo -a a b c
    The output should equal '["a","b","c"]'
  End

  It 'make object with subshell that make array'
    When call rjo name=gorilla likes="$(rjo -a a b c)"
    The output should equal '{"likes":["a","b","c"],"name":"gorilla"}'
  End

  It 'make array with subshell that make object'
    When call rjo -a a b "$(rjo name=gorilla)"
    The output should equal '["a","b",{"name":"gorilla"}]'
  End
End
