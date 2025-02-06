# Finanalyze
## Elevators Pitch
Our financial instrument analysis tool empowers individuals by providing comprehensive research into the current and historical trends of any instrument. Using advanced language models, we deliver clear, objective insights, keeping users informed. You stay in control and make confident financial decisions based on data-driven analysis.
## Project overview
The Finanalyze project is a web application that analyzes financial data using generative models and data analysis. The project focuses on programmatically searching for data, analyzing the data and visualizing it if useful. The user enters a financial instrument (Stock, Bond, Cryptocurrency, etc.) and gets an analysis of the data of that instrument in the form of a report. The program first generates a search term to find the data. Then the data is analyzed and visualized. The user can download the report as a PDF. The target group of the project are individuals who want to support their financial decisions with analysis.
> [!note] Our MVP is limited to top companies listed on either the NYSE or NASDAQ. We will expand to other financial instruments in the future.
## Problem Statement
The problem that our project, Finanalyze, aims to solve is the accessibility of financial analysis for individuals who want to invest in various financial instruments, such as stocks, bonds, and cryptocurrencies. Many investors, especially beginners, struggle to find relevant and reliable information that helps them make informed decisions. This lack of insight can lead to sub optimal investments and financial losses.
## Solution Overview
Our solution is the development of Finanalyze, an accessible web app that offers financial analysis and insights for all kinds of investments. The platform is specifically aimed at individual users who want to improve their investment choices, without having to delve deeply into the financial world or spend a lot of money on expensive analysis tools.
## Workflow
> Each step is a prompt that is given to the LLM. The LLM will generate a response to the prompt. The response is then parsed and used to generate the next prompt. The process continues until the report is generated.

> We use some keywords to indicate the type of prompt. `once` is used to indicate that the prompt is only given once. `each` is used to indicate that the prompt is given for each item in a list. Since we mostly work with strings in this workflow, we don't specify that variables are strings. We only specify the type if it is not a string, e.g. `boolean` or `[]`-suffix for a list.

> This process starts when the user clicks the `Generate`-button on the web app. At which point the user's input is passed to the backend where this flow starts.

> This section describes the flow of a user's request through the generation process. This section does not discuss UX/UI, API, or other technical details. It is an overview of the LLM/RAG workflow.
1. `once` Is this valid?
    1. `input`
        1. `user_input`
    2. `output`
        1. `valid`: `boolean`
2. `once` Generate a title for the report.
    1. `input`
        1. `user_input`
    2. `output`
        1. `generated_title`
3. `once` Generate first level section headings that should be included in the report.
    1. `input`
        1. `generated_title`
        2. `user_input`
    2. `output`
        1. `heading[]`
4. `each heading` generate paragraph level description of the content that should be in the paragraph in bullet points.
    1. `input`
        1. `heading`
    2. `output`
        1. `paragraph_bullet_point[]`
5. `each paragraph_bullet` Generate 4-6 search queries that you could google to find.
    1. `input`
        1. `report_title`
        2. `section_heading`
    2. `output`
        1. `query[4..6]`
6. `each query` look up and scrape top 5 results for html.
7. `each source` extract data from this text (tables), if there is none, `null`
    1. `input`
        1. `raw_content`
    2. `output`
        1. `csv`
8. `each source` extract content from this text (article), if there is none, `null`
    1. `input`
        1. `raw_content`
    2. `output`
        1. `sanatized_content_md`
9. `each content source(website,pdf)` generate a title and description for this source.
    1. `input`
        1. `sanatized_content_md`
        2. `url`
    2. `output`
        1. `title`
        2. `description`
10. `each data source(website table,pdf table,csv,excel)` generate a title and description for this source and it's data.
    1. `input`
        1. `surrounding_content_start`
        2. `data_head`
        3. `data_info`
        4. `surrounding_content_end`
    2. `output`
        1. `title`
        2. `description`
        3. `each column`
            1. `name`
            2. `type`
            3. `description`
11. `each paragraph bullet point` generate a chunk of text based on RAG'ed.
    1. `input`
        1. `heading`
        2. `heading_description`
        3. `paragraph_description`
        4. `paragraph_bullet_point`
        5. `ragged_source[]`
            1. `id`
            2. `name`
    2. `output`
        1. `paragraph_chunk`
        2. `source[]`
            1. `id`
            2. `name`
            3. `reason_used`
12. `combined paragraph bullet point chunks` combine the chunks of text into a single coherent paragraph.
    1. `input`
        1. `heading`
        2. `heading_description`
        3. `paragraph_description`
        4. `paragraph_bullet_point[]`
        5. `paragraph_bullet_point_chunk[]`
        6. `source[]`
            1. `id`
            2. `name`
            3. `reason_used`
    2. `output`
        1. `paragraph`
        2. `source[]`
            1. `id`
            2. `name`
            3. `reason_used`
13. `combined paragraphs` combine the paragraphs into a single coherent text under the heading.
    1. `input`
        1. `heading`
        2. `heading_description`
        3. `paragraphs[]`
        4. `source[]`
            1. `id`
            2. `name`
            3. `reason_used`
    2. `output`
        1. `heading_text`
        2. `source[]`
            1. `id`
            2. `name`
            3. `reason_used`
14. `each heading text` insert bibtex citations into the text based on the reason, `id` is the bibtex citation key.
    1. `input`
        1. `heading_text`
        2. `source[]`
            1. `id`
            2. `name`
            3. `reason`
    2. `output`
        1. `heading_text_with_citations`
15. `each heading text` what graphics could be inserted to make the text more informative?
    1. `input`
        1. `heading_text_with_citations`
    2. `output`
            1. `graphics[]`
                1. `type`
                2. `description`
                3. `caption`
16. `each generated graphic` insert the graphic into the text.
    1. `input`
            2. `heading_text_with_citations`
            3. `graphics[]`
                1. `path`
                2. `type`
                3. `description`
                4. `caption`
    2. `output`
        1. `heading_text_with_citations_and_graphics`
