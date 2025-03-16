# KVERS = $(shell uname -r)
KVERS = 6.13.7-arch1-1
PWD = $(shell pwd)

obj-m += globalmem.o

all: kernel_modules

kernel_modules:
	make -C /lib/modules/$(KVERS)/build M=$(PWD) modules 

clean:
	make -C /lib/modules/$(KVERS)/build M=$(PWD) clean