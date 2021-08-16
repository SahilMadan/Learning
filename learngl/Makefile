SHELL := /bin/bash

CC=g++
SRCDIR=./src
BUILDIR=./build
OBJDIR=./obj
FLAGS=-Wall -Werror -Wpedantic -Wextra -lglad -lglfw
SHADER_S=${OBJDIR}/shader_simple.o
SHADER_M=${OBJDIR}/shader_m.o
STB=-lstb

setup:
	@echo "Setting up..."
	mkdir -p ${BUILDIR}
	mkdir -p ${OBJDIR}
	@echo "Ready to build individual targets. For example, \"make 1_1\""

1_1: ${SRCDIR}/1_1_hello_window.cpp 
	${CC} ${SRCDIR}/1_1_hello_window.cpp \
		${FLAGS} -o ${BUILDIR}/1_1 &&\
		${BUILDIR}/1_1

1_2: ${SRCDIR}/1_2_hello_window_clear.cpp 
	${CC} ${SRCDIR}/1_2_hello_window_clear.cpp \
		${FLAGS} -o ${BUILDIR}/1_2 &&\
		${BUILDIR}/1_2

2_1: ${SRCDIR}/2_1_hello_triangle.cpp 
	${CC} ${SRCDIR}/2_1_hello_triangle.cpp \
		${FLAGS} -o ${BUILDIR}/2_1 &&\
		${BUILDIR}/2_1

2_2: ${SRCDIR}/2_2_hello_triangle_indexed.cpp 
	${CC} ${SRCDIR}/2_2_hello_triangle_indexed.cpp \
		${FLAGS} -o ${BUILDIR}/2_2 &&\
		${BUILDIR}/2_2

3_1: ${SRCDIR}/3_1_shaders_uniform.cpp 
	${CC} ${SRCDIR}/3_1_shaders_uniform.cpp \
		${FLAGS} -o ${BUILDIR}/3_1 &&\
		${BUILDIR}/3_1

3_2: ${SRCDIR}/3_2_shaders_interpolation.cpp 
	${CC} ${SRCDIR}/3_2_shaders_interpolation.cpp \
		${FLAGS} -o ${BUILDIR}/3_2 &&\
		${BUILDIR}/3_2

3_3: ${SRCDIR}/3_3_shaders_class.cpp shader_simple
	${CC} ${SRCDIR}/3_3_shaders_class.cpp ${SHADER_S} \
		${FLAGS} -o ${BUILDIR}/3_3 && \
		${BUILDIR}/3_3

4_1: ${SRCDIR}/4_1_textures.cpp shader_simple
	${CC} ${SRCDIR}/4_1_textures.cpp ${SHADER_S} \
		${FLAGS} ${STB} -o ${BUILDIR}/4_1 && \
		${BUILDIR}/4_1

4_2: ${SRCDIR}/4_2_textures_combined.cpp shader_simple
	${CC} ${SRCDIR}/4_2_textures_combined.cpp ${SHADER_S} \
		${FLAGS} ${STB} -o ${BUILDIR}/4_2 && \
		${BUILDIR}/4_2

5_1: ${SRCDIR}/5_1_transformations.cpp shader_m
	${CC} ${SRCDIR}/5_1_transformations.cpp ${SHADER_M} \
		${FLAGS} ${STB} -o ${BUILDIR}/5_1 && \
		${BUILDIR}/5_1

6_1: ${SRCDIR}/6_1_coordinate_systems.cpp shader_m
	${CC} ${SRCDIR}/6_1_coordinate_systems.cpp ${SHADER_M} \
		${FLAGS} ${STB} -o ${BUILDIR}/6_1 && \
		${BUILDIR}/6_1

6_2: ${SRCDIR}/6_2_coordinate_systems_depth.cpp shader_m
	${CC} ${SRCDIR}/6_2_coordinate_systems_depth.cpp ${SHADER_M} \
		${FLAGS} ${STB} -o ${BUILDIR}/6_2 && \
		${BUILDIR}/6_2

shader_simple: ${SRCDIR}/shader_simple.cpp
	${CC} ${SRCDIR}/shader_simple.cpp \
		${FLAGS} -c -o ${SHADER_S} 

shader_m: ${SRCDIR}/shader_m.cpp
	${CC} ${SRCDIR}/shader_m.cpp \
		${FLAGS} -c -o ${SHADER_M} 

clean:
	rm -rf ${BUILDIR}
	rm -rf ${OBJDIR}	
