SHELL := /bin/bash

CC=g++
DEFAULTLIBS=-lglad -lglfw
BUILDIR=./build
FLAGS=-Wall -Werror -Wpedantic -Wextra

build_dir:
	mkdir -p ${BUILDIR}

1_1: 1_1_hello_window.cpp 

	${CC} 1_1_hello_window.cpp ${FLAGS} ${DEFAULTLIBS} -o ${BUILDIR}/1_1 &&\
		${BUILDIR}/1_1

1_2: 1_2_hello_window_clear.cpp 
	${CC} 1_2_hello_window_clear.cpp ${FLAGS} ${DEFAULTLIBS} -o ${BUILDIR}/1_2 &&\
		${BUILDIR}/1_2

2_1: 2_1_hello_triangle.cpp 
	${CC} 2_1_hello_triangle.cpp ${FLAGS} ${DEFAULTLIBS} -o ${BUILDIR}/2_1 &&\
		${BUILDIR}/2_1

2_2: 2_2_hello_triangle_indexed.cpp 
	${CC} 2_2_hello_triangle_indexed.cpp ${FLAGS} ${DEFAULTLIBS} -o ${BUILDIR}/2_2 &&\
		${BUILDIR}/2_2

3_1: 3_1_shaders_uniform.cpp 
	${CC} 3_1_shaders_uniform.cpp ${FLAGS} ${DEFAULTLIBS} -o ${BUILDIR}/3_1 &&\
		${BUILDIR}/3_1

3_2: 3_2_shaders_interpolation.cpp 
	${CC} 3_2_shaders_interpolation.cpp ${FLAGS} ${DEFAULTLIBS} -o ${BUILDIR}/3_2 &&\
		${BUILDIR}/3_2

clean:
	rm -rf ${BUILDIR}/*
	
