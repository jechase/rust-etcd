#!/usr/bin/env bash
#
# Generate all etcd protobuf bindings.
# Run from repository root.
#
set -e

if ! [[ "$0" =~ genproto.sh ]]; then
	echo "must be run from repository root"
	exit 255
fi

cd etcd

if [[ $(protoc --version | cut -f2 -d' ') != "3.5.1" ]]; then
	echo "could not find protoc 3.5.1, is it installed + in PATH?"
	exit 255
fi

# directories containing protos to be built
DIRS="./wal/walpb ./etcdserver/etcdserverpb ./snap/snappb ./raft/raftpb ./mvcc/mvccpb ./lease/leasepb ./auth/authpb ./etcdserver/api/v3lock/v3lockpb ./etcdserver/api/v3election/v3electionpb"

# exact version of packages to build
GOGO_PROTO_SHA="41168f6614b7bb144818ec8967b8c702705df564"
GRPC_GATEWAY_SHA="a92d37fb6339375fa4bb7d9c364f92373fe199c3"
SCHWAG_SHA="b7d0fc9aadaaae3d61aaadfc12e4a2f945514912"

# set up self-contained GOPATH for building
export GOPATH=${PWD}/gopath.proto
export GOBIN=${PWD}/bin
export PATH="${GOBIN}:${PATH}"
export ROOT=${PWD}/..

COREOS_ROOT="${GOPATH}/src/github.com/coreos"
ETCD_ROOT="${COREOS_ROOT}/etcd"
GOGOPROTO_ROOT="${GOPATH}/src/github.com/gogo/protobuf"
SCHWAG_ROOT="${GOPATH}/src/github.com/hexfusion/schwag"
GOGOPROTO_PATH="${GOGOPROTO_ROOT}:${GOGOPROTO_ROOT}/protobuf"
GRPC_GATEWAY_ROOT="${GOPATH}/src/github.com/grpc-ecosystem/grpc-gateway"

rm -f "${ETCD_ROOT}"
mkdir -p "${COREOS_ROOT}"
ln -s "${PWD}" "${ETCD_ROOT}"

# Ensure we have the right version of protoc-gen-gogo by building it every time.
# TODO(jonboulle): vendor this instead of `go get`ting it.
go get -u github.com/gogo/protobuf/{proto,protoc-gen-gogo,gogoproto}
go get -u golang.org/x/tools/cmd/goimports
pushd "${GOGOPROTO_ROOT}"
	git reset --hard "${GOGO_PROTO_SHA}"
	make install
popd

# generate gateway code
go get -u github.com/grpc-ecosystem/grpc-gateway/protoc-gen-grpc-gateway
go get -u github.com/grpc-ecosystem/grpc-gateway/protoc-gen-swagger
pushd "${GRPC_GATEWAY_ROOT}"
	git reset --hard "${GRPC_GATEWAY_SHA}"
	go install ./protoc-gen-grpc-gateway
popd

for dir in ${DIRS}; do
	pushd "${dir}"
		protoc --rust_out=${ROOT}/src/v3 -I=".:${GOGOPROTO_PATH}:${COREOS_ROOT}:${GRPC_GATEWAY_ROOT}/third_party/googleapis" ./*.proto
		protoc --rust-grpc_out=${ROOT}/src/v3 -I=".:${GOGOPROTO_PATH}:${COREOS_ROOT}:${GRPC_GATEWAY_ROOT}/third_party/googleapis" ./*.proto
	popd
done

echo Done

exit 0
