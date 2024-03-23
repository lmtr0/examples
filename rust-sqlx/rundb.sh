podman run --name yudb -p7000:7000 -p9000:9000 -p15433:15433 -p5433:5433 -p9042:9042 \
    yugabytedb/yugabyte:2.20.2.1-b3 bin/yugabyted start \
    --background=false
