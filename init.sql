--
-- PostgreSQL database dump
--

-- Dumped from database version 11.1 (Debian 11.1-1.pgdg90+1)
-- Dumped by pg_dump version 11.1 (Debian 11.1-1.pgdg90+1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_with_oids = false;

\c postgres;

DROP DATABASE IF EXISTS d4g;

CREATE DATABASE d4g;
ALTER DATABASE d4g OWNER TO d4g;

\c d4g;

--
-- Name: logement; Type: TABLE; Schema: public; Owner: d4g
--

CREATE TABLE public.logement (
    foyer character varying(16) NOT NULL,
    type integer NOT NULL,
    surface real NOT NULL,
    nb_pieces integer NOT NULL,
    chauffage character varying(16) NOT NULL,
    date_construction integer NOT NULL,
    n_voie character varying(8) NOT NULL,
    voie1 character varying(254) NOT NULL,
    code_postal character varying(5) NOT NULL,
    ville character varying(128) NOT NULL
);


ALTER TABLE public.logement OWNER TO d4g;

--
-- Name: proprietaire; Type: TABLE; Schema: public; Owner: d4g
--

CREATE TABLE public.proprietaire (
    foyer character varying(16) NOT NULL,
    nom character varying(128),
    prenom character varying(128),
    societe character varying(128),
    adresse character varying(254)
);


ALTER TABLE public.proprietaire OWNER TO d4g;

--
-- Name: locataire; Type: TABLE; Schema: public; Owner: d4g
--

CREATE TABLE public.locataire (
    foyer character varying(16) NOT NULL,
    nom character varying(128) NOT NULL,
    prenom character varying(128) NOT NULL
);


ALTER TABLE public.locataire OWNER TO d4g;

--
-- Name: releve; Type: TABLE; Schema: public; Owner: d4g
--

CREATE TABLE public.releve (
    foyer character varying(16) NOT NULL,
    date character varying(10) NOT NULL,
    valeur integer NOT NULL
);


ALTER TABLE public.releve OWNER TO d4g;

--
-- Name: utilisateur; Type: TABLE; Schema: public; Owner: d4g
--

CREATE TABLE public.utilisateur (
    foyer character varying(16) NOT NULL,
    login character varying(64) NOT NULL,
    password character varying(255) NOT NULL,
    active boolean NOT NULL,
    token character varying(21),
    admin boolean NOT NULL
);

ALTER TABLE public.utilisateur OWNER TO d4g;

CREATE TABLE public.fichier (
    foyer character varying(16) NOT NULL,
    id character varying(255) NOT NULL,
    file_path character varying (255) NOT NULL
);

ALTER TABLE public.fichier OWNER TO d4g;

--
-- Data for Name: locataire; Type: TABLE DATA; Schema: public; Owner: d4g
--

COPY public.locataire (foyer, nom, prenom) FROM stdin;
\.


--
-- Data for Name: logement; Type: TABLE DATA; Schema: public; Owner: d4g
--

COPY public.logement (foyer, type, surface, nb_pieces, chauffage, date_construction, n_voie, voie1, code_postal, ville) FROM stdin;
\.


--
-- Data for Name: proprietaire; Type: TABLE DATA; Schema: public; Owner: d4g
--

COPY public.proprietaire (foyer, nom, prenom, societe, adresse) FROM stdin;
\.


--
-- Data for Name: releve; Type: TABLE DATA; Schema: public; Owner: d4g
--

COPY public.releve (date, foyer, valeur) FROM stdin;
\.


--
-- Data for Name: utilisateur; Type: TABLE DATA; Schema: public; Owner: d4g
--

COPY public.utilisateur (foyer, login, password, active, admin) FROM stdin;
\.


COPY public.fichier (id, foyer, file_path) FROM stdin;
\.
--
-- Name: locataire locataire_pkey; Type: CONSTRAINT; Schema: public; Owner: d4g
--

ALTER TABLE ONLY public.locataire
    ADD CONSTRAINT locataire_pkey PRIMARY KEY (foyer);


--
-- Name: logement logement_pkey; Type: CONSTRAINT; Schema: public; Owner: d4g
--

ALTER TABLE ONLY public.logement
    ADD CONSTRAINT logement_pkey PRIMARY KEY (foyer);


--
-- Name: proprietaire proprietaire_pkey; Type: CONSTRAINT; Schema: public; Owner: d4g
--

ALTER TABLE ONLY public.proprietaire
    ADD CONSTRAINT proprietaire_pkey PRIMARY KEY (foyer);


--
-- Name: releve releve_pkey; Type: CONSTRAINT; Schema: public; Owner: d4g
--

ALTER TABLE ONLY public.releve
    ADD CONSTRAINT releve_pkey PRIMARY KEY (date, foyer);


--
-- Name: utilisateur utilisateur_pkey; Type: CONSTRAINT; Schema: public; Owner: d4g
--

ALTER TABLE ONLY public.utilisateur
    ADD CONSTRAINT utilisateur_pkey PRIMARY KEY (login);

--

ALTER TABLE ONLY public.fichier
    ADD CONSTRAINT fichier_pkey PRIMARY KEY (id);

-- Name: proprietaire logement_proprietaire_fk; Type: FK CONSTRAINT; Schema: public; Owner: d4g
--

ALTER TABLE ONLY public.proprietaire
    ADD CONSTRAINT logement_proprietaire_fk FOREIGN KEY (foyer) REFERENCES public.logement(foyer) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: releve logement_releve_fk; Type: FK CONSTRAINT; Schema: public; Owner: d4g
--

ALTER TABLE ONLY public.releve
    ADD CONSTRAINT logement_releve_fk FOREIGN KEY (foyer) REFERENCES public.logement(foyer) ON UPDATE CASCADE ON DELETE CASCADE;


ALTER TABLE ONLY public.fichier
    ADD CONSTRAINT logement_fichier_fk FOREIGN KEY (foyer) REFERENCES public.logement(foyer) ON UPDATE CASCADE ON DELETE CASCADE;


--
