vai() {
 node=${1}
 #node="node488001.tari-rpc.tari.com"
 ${curl} -k -X POST https://${node}/services/kwrapper/plugins \
    --data "name=cors"  \
    --data "config.origins='*'" \
    --data "config.methods=GET" \
    --data "config.methods=POST" \
    --data "config.headers=Accept" \
    --data "config.headers=Accept-Version" \
    --data "config.headers=Content-Length" \
    --data "config.headers=Content-MD5" \
    --data "config.headers=Content-Type" \
    --data "config.headers=Date" \
    --data "config.headers=X-Auth-Token" \
    --data "config.exposed_headers=X-Auth-Token" \
    --data "config.credentials=true" \
    --data "config.max_age=3600"
}

for n in "node487001.tari-rpc.tari.com node488001.tari-rpc.tari.com node489001.tari-rpc.tari.com"; do
 vai node488001.tari-rpc.tari.com
done
