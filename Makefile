# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: qdequele <qdequele@student.42.fr>          +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2015/12/15 14:57:05 by qdequele          #+#    #+#              #
#    Updated: 2017/11/13 16:02:16 by qdequele         ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

all:
	@cargo build --release
	@cp target/release/estimate .
	@cp target/release/learn .

clean:
	@/bin/rm -rf target
	@echo $(NAME) " - Clean all .o files"

fclean: clean
	@/bin/rm -rf learn estimate .data
	@echo $(NAME) " - Clean executable"

re: fclean all
