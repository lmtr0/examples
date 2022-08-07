DEPS = $(shell pkg-config --libs --cflags gstreamer-1.0)

default:
	${CC} helloworld.c -o helloworld ${DEPS}
	${CC} streaming.c -o streaming ${DEPS}
