{ 
  writeShellScriptBin,
  audience ? "world", 
}:
writeShellScriptBin "hello" ''
  echo "Hello, ${audience}!"
''