--
-- PostgreSQL database dump
--

-- Dumped from database version 17.2
-- Dumped by pg_dump version 17.2

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: adminview; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.adminview AS
 SELECT table_catalog,
    table_schema,
    table_name,
    table_type,
    self_referencing_column_name,
    reference_generation,
    user_defined_type_catalog,
    user_defined_type_schema,
    user_defined_type_name,
    is_insertable_into,
    is_typed,
    commit_action
   FROM information_schema.tables
  WHERE ((table_schema)::name = 'Globacom'::name);


ALTER VIEW public.adminview OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_name character varying(50) NOT NULL,
    c_age integer,
    c_email character varying(100),
    c_mobile character varying(20),
    eid integer NOT NULL,
    data_id integer
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Name: customerview; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.customerview AS
 SELECT c_id,
    c_name,
    c_age,
    c_email,
    c_mobile,
    eid,
    data_id
   FROM public.customer;


ALTER VIEW public.customerview OWNER TO postgres;

--
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer NOT NULL,
    data_size character varying(10),
    data_duration integer,
    data_price integer
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Name: projectmanagerview; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.projectmanagerview AS
 SELECT data_id,
    data_size,
    data_duration,
    data_price
   FROM public.dataplan;


ALTER VIEW public.projectmanagerview OWNER TO postgres;

--
-- Name: vendorview; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.vendorview AS
 SELECT data_id,
    data_size,
    data_duration,
    data_price
   FROM public.dataplan;


ALTER VIEW public.vendorview OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (c_id, c_name, c_age, c_email, c_mobile, eid, data_id) FROM stdin;
110	Musta Karim	35	m_karim@gmail.com	08055089112	102	5
111	Lilian Jaiya	43	l_jaiye@gmail.com	08055185341	100	3
112	Arthur Musa	50	a_musa@gmail.com	07055282813	107	10
113	Philip Akonjo	41	p_akonjo@gmail.com	09052356772	100	2
114	Marylene Mapa	33	m_mapa@gmail.com	08053333551	120	5
115	Oghenero Agor	50	o_agor@gmail.com	07055566774	117	11
116	Adams Bree	33	a_bree@gmail.com	08056765424	120	1
117	Okafor Mathias	45	o_mathias@gmail.com	08056763367	120	10
118	Samson Adeleke	65	s_adeleke@gmail.com	07056774423	117	11
119	Lawal Tamire	38	l_tamire@gmail.com	09051191101	100	8
120	James Job	44	j_job@gmail.com	08059693919	120	7
121	Matthew Jakande	21	m_jakande@gmail.com	07051232144	100	2
122	Jimila Adegboye	20	j_adegboye@gmail.com	08054921923	117	5
\.


--
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (data_id, data_size, data_duration, data_price) FROM stdin;
1	350MB	2	200
2	1.8GB	14	500
3	3.9GB	30	1000
4	7.5GB	30	1500
5	9.2GB	30	2000
6	10.8GB	30	2500
7	14GB	30	3000
8	18GB	30	4000
9	24GB	30	5000
10	29.9GB	30	8000
11	50GB	30	10000
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- Name: dataplan dataplan_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_pkey PRIMARY KEY (data_id);


--
-- Name: customer customer_data_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_data_id_fkey FOREIGN KEY (data_id) REFERENCES public.dataplan(data_id);


--
-- Name: TABLE customer; Type: ACL; Schema: public; Owner: postgres
--

GRANT SELECT ON TABLE public.customer TO customer;


--
-- Name: TABLE dataplan; Type: ACL; Schema: public; Owner: postgres
--

GRANT SELECT ON TABLE public.dataplan TO project_manager;
GRANT SELECT ON TABLE public.dataplan TO vendor;


--
-- PostgreSQL database dump complete
--

