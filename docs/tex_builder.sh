# using dockerised version makes it easy
docker pull miktex/miktex

build(){
      docker run --rm -ti \
        -v miktex:/miktex/.miktex \
        -v $(pwd):/miktex/work \
        -e MIKTEX_GID=$(id -g) \
        -e MIKTEX_UID=$(id -u) \
        miktex/miktex \
        pdflatex ${file}; bibtex ${file}; pdflatex ${file}; pdflatex ${file};
}


if [ -z "$1" ] 
then 
  echo "Currently avalible files:"
  for fname in *.tex;do echo $fname;done
  read -p "which file do you want to compile (press ENTER to compile all) " file
  if [ -z "$file" ]
  then
    for name in *.tex
      do  file=$name; 
      build
    done
  else
      build file
  fi
else
  file=$1
  build file
fi
for ex in aux bbl blg fls lof log lot out toc synctex.gz fdb_latexmk
do 
  for name in *.$ex
  do rm $name; 
  done
done

echo "Script finished with success :)"