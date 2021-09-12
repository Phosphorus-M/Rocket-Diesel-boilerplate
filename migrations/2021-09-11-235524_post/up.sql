-- Your SQL goes here

DROP TABLE IF EXISTS public.post;

CREATE TABLE public.post (
	t_id SERIAL NOT NULL,
	title varchar(70) NOT NULL,
	message text NOT NULL,
	views int4 DEFAULT 0,
	crated_on timestamp DEFAULT CURRENT_TIMESTAMP,
	author_id int4 NOT NULL,
	CONSTRAINT posts_pkey PRIMARY KEY (t_id)
);

INSERT INTO public.post (title, message, views, crated_on, author_id) VALUES ('Post 1', 'Post 1 message', 0, '2020-09-01 18:27:36.907', 1);
INSERT INTO public.post (title, message, views, crated_on, author_id) VALUES ('Post 2', 'Post 2 message', 0, '2020-09-02 18:27:36.907', 1);
INSERT INTO public.post (title, message, views, crated_on, author_id) VALUES ('Post 3', 'Post 3 message', 0, '2020-09-03 18:27:36.907', 1);
INSERT INTO public.post (title, message, views, crated_on, author_id) VALUES ('Post 4', 'Post 4 message', 0, '2020-09-04 18:27:36.907', 1);
INSERT INTO public.post (title, message, views, crated_on, author_id) VALUES ('Post 5', 'Post 5 message', 0, '2020-09-05 18:27:36.907', 1);
INSERT INTO public.post (title, message, views, crated_on, author_id) VALUES ('Post 6', 'Post 6 message', 0, '2020-09-06 18:27:36.907', 1);
INSERT INTO public.post (title, message, views, crated_on, author_id) VALUES ('Post 7', 'Post 7 message', 0, '2020-09-07 18:27:36.907', 1);
INSERT INTO public.post (title, message, views, crated_on, author_id) VALUES ('Post 8', 'Post 8 message', 0, '2020-09-08 18:27:36.907', 1);
INSERT INTO public.post (title, message, views, crated_on, author_id) VALUES ('Post 9', 'Post 9 message', 0, '2020-09-09 18:27:36.907', 1);