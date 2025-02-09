<script lang="ts">
    import { Button } from '$lib/components/ui/button/index.js';

    import { goto } from '$app/navigation';
</script>

<div class="contact-page-wrapper bg-black text-white min-h-screen">
    <!-- Navbar -->
    <header class="flex items-center justify-between px-8 py-4">
        <div class="text-lg font-bold">
            <img src="/favicon.svg" alt="Logo" class="h-8" /> <!-- Update the path to your logo -->
        </div>
        <nav class="flex gap-6">
            <a href="/" class="hover:text-purple-400">Home</a>
            <a href="/about" class="hover:text-purple-400">About</a>
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

    <!-- Contact Section -->
    <section class="contact py-16 px-8">
        <h1 class="text-5xl font-bold mb-6 text-center">Contact Us</h1>
        <div class="max-w-4xl mx-auto text-lg text-gray-300">
            <p class="mb-6">
                Have questions or need assistance? Our team is here to help. Reach out to us using the form below or through our direct contact channels. We’re committed to providing prompt and effective support to ensure your experience is seamless.
            </p>
        </div>

        <!-- Contact Form -->
        <form class="bg-gray-800 p-8 rounded-lg shadow-md max-w-3xl mx-auto">
            <div class="mb-6">
                <label for="name" class="block text-gray-400 mb-2">Name</label>
                <input type="text" id="name" name="name" class="w-full px-4 py-2 bg-gray-900 text-white rounded-lg focus:ring-2 focus:ring-purple-400" placeholder="Enter your name" required />
            </div>

            <div class="mb-6">
                <label for="email" class="block text-gray-400 mb-2">Email</label>
                <input type="email" id="email" name="email" class="w-full px-4 py-2 bg-gray-900 text-white rounded-lg focus:ring-2 focus:ring-purple-400" placeholder="Enter your email" required />
            </div>

            <div class="mb-6">
                <label for="message" class="block text-gray-400 mb-2">Message</label>
                <textarea id="message" name="message" rows="5" class="w-full px-4 py-2 bg-gray-900 text-white rounded-lg focus:ring-2 focus:ring-purple-400" placeholder="Write your message here..." required></textarea>
            </div>

            <div class="text-center">
                <Button type="submit" class="bg-purple-700 hover:bg-purple-600 px-6 py-3 text-lg">Send Message</Button>
            </div>
        </form>
    </section>

    <!-- Footer Section -->
    <footer class="bg-gray-900 text-gray-400 py-6 text-center mt-16">
        <p>&copy; {new Date().getFullYear()} FinAn. All rights reserved.</p>
    </footer>
</div>

<style>
    .contact-page-wrapper {
        background: linear-gradient(to bottom, #1a1a1a, #000000);
    }

    h1 {
        color: #c084fc;
    }

    p {
        line-height: 1.8;
    }

    form input,
    form textarea {
        border: none;
        outline: none;
    }
</style>
