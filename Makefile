# clifi makefile

CPP = g++ -std=c++17 -Wall -Wno-unused-variable -I include -c
LD = g++
CPPDEBUG = g++ -std=c++17 -g -Wall -Wextra -pedantic -I include -c
INCLUDES = 

CL := \033[1;34m
NC := \033[0m

folder := src/
files := clifi.cpp

.SILENT all:
	for file in $(files); do \
		echo -e " 	$(CL)CPP $$file$(NC)" ; \
		$(CPP) $(folder)$$file ; \
	done
	echo  -e " 	$(CL)LD *.o$(NC)"
	$(LD) $(INCLUDES) *.o -o clifi
	echo "Cleaning up..."
	rm *.o
	echo "Done!"

debug:
	for file in $(files); do \
		echo  -e " 	$(CL)CPPDEBUG $$file$(NC)" ; \
		$(CPPDEBUG) $(folder)$$file ; \
	done
	echo  -e " 	$(CL)LD *.o$(NC)"
	$(LD) $(INCLUDES) *.o -o clifi
	echo "Cleaning up..."
	rm *.o
	echo "Done!"