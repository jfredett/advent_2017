echo -n 'watchman-make '
for bin in `ls src/bin/` ; do 
  echo "-p 'src/bin/$bin' -t 'test --bin ${bin%*.rs}' --make 'clear ; cargo' \\"; 
done
echo '-p Cargo.toml -t update --make "clear ; cargo"' 
