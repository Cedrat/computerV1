all :
ifneq (,$(wildcard ./computer_v1))
	rm computer_v1
endif
	cargo build
	mv target/debug/computer_v1  computer_v1

clean :
	cargo clean

fclean :
ifneq (,$(wildcard ./computer_v1))
	rm computer_v1
endif
	cargo clean

re : 
ifneq (,$(wildcard ./computer_v1))
	rm computer_v1
endif
	cargo clean
	cargo build
	mv target/debug/computer_v1  computer_v1
	