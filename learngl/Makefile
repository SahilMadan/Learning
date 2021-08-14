SHELL := /bin/bash

CC=g++
DEFAULTLIBS=-lglad -lglfw
SRCDIR=./src
BUILDIR=./build
OBJDIR=./obj
FLAGS=-Wall -Werror -Wpedantic -Wextra
SHADER_S=${OBJDIR}/shader_simple.o


setup:
	@echo "Setting up..."
	mkdir -p ${BUILDIR}
	mkdir -p ${OBJDIR}
	@echo "Ready to build individual targets. For example, \"make 1_1\""

1_1: ${SRCDIR}/1_1_hello_window.cpp 
	${CC} ${SRCDIR}/1_1_hello_window.cpp \
		${FLAGS} ${DEFAULTLIBS} -o ${BUILDIR}/1_1 &&\
		${BUILDIR}/1_1

1_2: ${SRCDIR}/1_2_hello_window_clear.cpp 
	${CC} ${SRCDIR}/1_2_hello_window_clear.cpp \
		${FLAGS} ${DEFAULTLIBS} -o ${BUILDIR}/1_2 &&\
		${BUILDIR}/1_2

2_1: ${SRCDIR}/2_1_hello_triangle.cpp 
	${CC} ${SRCDIR}/2_1_hello_triangle.cpp \
		${FLAGS} ${DEFAULTLIBS} -o ${BUILDIR}/2_1 &&\
		${BUILDIR}/2_1

2_2: ${SRCDIR}/2_2_hello_triangle_indexed.cpp 
	${CC} ${SRCDIR}/2_2_hello_triangle_indexed.cpp \
		${FLAGS} ${DEFAULTLIBS} -o ${BUILDIR}/2_2 &&\
		${BUILDIR}/2_2

3_1: ${SRCDIR}/3_1_shaders_uniform.cpp 
	${CC} ${SRCDIR}/3_1_shaders_uniform.cpp \
		${FLAGS} ${DEFAULTLIBS} -o ${BUILDIR}/3_1 &&\
		${BUILDIR}/3_1

3_2: ${SRCDIR}/3_2_shaders_interpolation.cpp 
	${CC} ${SRCDIR}/3_2_shaders_interpolation.cpp \
		${FLAGS} ${DEFAULTLIBS} -o ${BUILDIR}/3_2 &&\
		${BUILDIR}/3_2

3_3: ${SRCDIR}/3_3_shaders_class.cpp shader_simple
	${CC} ${SRCDIR}/3_3_shaders_class.cpp ${SHADER_S} \
		${FLAGS} ${DEFAULTLIBS} -o ${BUILDIR}/3_3 && \
		${BUILDIR}/3_3

shader_simple: ${SRCDIR}/shader_simple.cpp
	${CC} ${SRCDIR}/shader_simple.cpp \
		${FLAGS} ${DEFAULTLIBS} -c -o ${SHADER_S}


clean:
	rm -rf ${BUILDIR}
	rm -rf ${OBJDIR}	
