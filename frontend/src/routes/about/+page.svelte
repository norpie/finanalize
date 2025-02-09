<script lang="ts">
    import { Button } from '$lib/components/ui/button/index.js';

    import { goto } from '$app/navigation';
</script>


    <!-- Navbar -->
    <header class="flex items-center justify-between px-8 py-4 ">
        <div class="text-lg font-bold">
            <img src="/favicon.svg" alt="Logo" class="h-8" /> <!-- Update the path to your logo -->
        </div>
        <nav class="flex gap-6">
            <a href="/" class="hover:text-purple-400">Home</a>
            <a href="/contact" class="hover:text-purple-400">Contact</a>
            <div class="relative">
                <button class="hover:text-purple-400">ENG ▼</button>
            </div>
        </nav>
        <div class="flex gap-2">
            
            <a href="/login">
                <Button class="bg-gray-700 hover:bg-gray-600">Log In</Button>
            </a>
            <a href="/register">
                <Button class="bg-purple-700 hover:bg-purple-600">Sign Up</Button>
            </a>
        </div>
    </header>
<div class="about-page-wrapper bg-gray-900">

    <!-- About Section -->
    <section class="about py-16 px-8">
        <h1 class="text-5xl font-bold mb-6 text-center">About Our Project</h1>
        <div class="max-w-4xl mx-auto text-lg text-gray-300">
            <p class="mb-6">
                Welcome to our innovative platform that brings financial expertise to your fingertips. Our mission is to simplify access to critical financial data, empowering users to make informed decisions with confidence. With a sleek and intuitive interface, we deliver precise insights tailored to your needs.
            </p>
            <p class="mb-6">
                Our project combines cutting-edge technology with user-centric design. Powered by advanced AI algorithms, our tools enable you to analyze financial trends, discover insights, and access detailed reports from top companies worldwide. Whether you're an investor, a financial enthusiast, or simply curious, we have something for you.
            </p>
            <p>
                We are dedicated to providing reliable and accessible tools for everyone. Our team believes that transparency and innovation drive success, and we strive to embody these values in every feature we offer.

            </p>
            <p class="mb-6">
                The objective of this project is to create an intuitive and innovative platform that simplifies the process of accessing financial data and insights. By leveraging cutting-edge AI technology, the platform empowers users with actionable intelligence, whether for personal, professional, or investment purposes. Designed with transparency and user experience in mind, the project aims to make financial expertise more accessible, enabling users to analyze trends, discover key insights, and make well-informed decisions with ease. Our commitment is to bridge the gap between complex data and everyday users, fostering a deeper understanding of the financial landscape.
            </p>
        </div>
    </section>

    <!-- Call to Action -->
    <section class="call-to-action py-16 ">
        <div class="text-center">
            <h2 class="text-4xl font-bold text-purple-400 mb-4">Ready to Explore?</h2>
            <p class="text-gray-300 mb-8">
                Dive into the world of financial insights and take the first step towards smarter decisions.
            </p>
            <Button onclick={() => goto('/signup')} class="bg-purple-700 hover:bg-purple-600 text-lg px-6 py-3">
                Get Started Now
            </Button>
        </div>
    </section>

    <!-- Footer Section -->
    <footer class="bg-gray-900 text-gray-400 py-6 text-center mt-16">
        <p>&copy; {new Date().getFullYear()} FinAn. All rights reserved.</p>
    </footer>
</div>

<style>
    .about-page-wrapper {
        background: linear-gradient(to bottom, #1a1a1a, #000000);
    }

    h1, h2 {
        color: #c084fc;
    }

    p {
        line-height: 1.8;
    }

    .call-to-action {
        background: linear-gradient(to bottom, #2d2d2d, #1a1a1a);
    }
</style>
