using dockerised version makes it easy
docker pull miktex/miktex

if [ -z "$1" ] 
then 
  echo "Currently avalible files:"
  for fname in *.tex;do echo $fname;done
  read -p "which file do you want to compile (press ENTER to compile all) " file
  if [ -z "$file" ]
  then
    for name in *.tex
      do  file=$name; 
      docker run --rm -ti \
        -v miktex:/miktex/.miktex \
        -v $(pwd):/miktex/work \
        -e MIKTEX_GID=$(id -g) \
        -e MIKTEX_UID=$(id -u) \
        miktex/miktex \
        pdflatex ${file}; bibtex ${file}; pdflatex ${file}; pdflatex ${file};
    done
  else
    docker run --rm -ti \
    -v miktex:/miktex/.miktex \
    -v $(pwd):/miktex/work \
    -e MIKTEX_GID=$(id -g) \
    -e MIKTEX_UID=$(id -u) \
    miktex/miktex \
    pdflatex ${file}; bibtex ${file}; pdflatex ${file}; pdflatex ${file};
  fi
else
  docker run --rm -ti \
    -v miktex:/miktex/.miktex \
    -v $(pwd):/miktex/work \
    -e MIKTEX_GID=$(id -g) \
    -e MIKTEX_UID=$(id -u) \
    miktex/miktex \
    pdflatex ${1}; bibtex ${1}; pdflatex ${1}; pdflatex ${1};
fi
for ex in aux bbl blg fls lof log lot out toc synctex.gz fdb_latexmk
do 
  for name in *.$ex
  do rm $name; 
  done
done

echo "Script finished with success :)"