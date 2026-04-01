#!/bin/bash
echo "Setup running..."
mntdir=${MOUNT_DIR:-.}
echo "Mount Dir: $mntdir"
rm ./run.sh
# "Read semaphore file, its filename is <dt>.todo, where <dt> is the datetime in format YYYY-MM-DDTHH-MM-SS, eg. 2024-06-01T12-00-00.todo"
echo "Read <datetime>.todo semaphore file to get datetime for processing"
semaphore_file=$(ls "$mntdir"/jobs/*.todo 2>/dev/null | head -n 1)
if [ -z "$semaphore_file" ]; then
  echo "No semaphore file found. Exiting."
  exit 1
fi
datetime=$(basename "$semaphore_file" .todo)
echo "Datetime for processing: $datetime"
for filepath in "$mntdir"/config/*.rx.txt; do
  if [ -f "$filepath" ]; then
	  file="$(basename "$filepath")"
    echo "basename $file"
	  issuer="${file%%.*}"
	  echo "format2 --config $filepath --input-dir $mntdir/output/$issuer/$datetime --output-dir $mntdir/output/formatted/$issuer/$datetime.json -f ndjson -l 3000 2>>$semaphore_file" >> ./run.sh
  fi
done
chmod +x run.sh
echo "Setup completed: $(cat ./run.sh)"
./run.sh
echo "Run completed."
echo "Rename semaphore file $semaphore_file"
mv $semaphore_file $mntdir/jobs/$datetime.done