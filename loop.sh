#"cargo test -- --show-output --test-threads=max" ;

for i in {0..500};
do
  eval "cargo test -- --show-output" ;
  if [[ $? -ne 0 ]];
    then break;    
  fi
  
done

echo $i
