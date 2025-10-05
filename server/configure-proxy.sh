if [ $# -lt 1 ]
then
        echo "Invalid arguments"
        exit 1
fi

rm /etc/nginx/stream.d/*

export host=$1
shift 1

for p in $*
do
        export name=port$p
        export port=$p
        envsubst '$host,$name,$port' < template.conf > /etc/nginx/stream.d/$name.conf
done

service nginx restart