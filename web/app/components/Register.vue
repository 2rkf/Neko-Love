<template>
  <div
    class="min-h-screen flex flex-col bg-gradient-to-b from-pink-100 via-pink-50 to-rose-100 text-zinc-800 font-[Nunito] noselect"
  >
    <Navbar />

    <main class="flex-grow flex items-center justify-center px-4 py-16">
      <UCard
        class="w-full max-w-md shadow-xl border border-pink-200 bg-white/80 backdrop-blur rounded-xl animate-fade-in-up"
        :ui="{ body: 'p-8 space-y-6 text-left' }"
      >
        <!-- Header -->
        <div class="text-center space-y-2">
          <h2 class="text-4xl font-extrabold text-pink-600 drop-shadow-pink">
            Create Account
          </h2>
          <p class="text-pink-700 text-sm">
            Create your account to join Neko-Love.
          </p>
        </div>

        <!-- Form -->
        <form class="space-y-4" @submit.prevent="submitForm">
          <!-- Username -->
          <div class="space-y-2">
            <label class="text-base font-semibold text-pink-600"
              >Username</label
            >
            <UInput
              v-model="username"
              size="lg"
              placeholder="Choose a username"
              color="primary"
              class="w-full rounded-xl"
              :ui="{
                base: 'bg-white text-pink-700 placeholder-pink-300',
                wrapper:
                  'focus-within:ring-2 focus-within:ring-pink-300 rounded-xl',
              }"
            />
          </div>

          <!-- Email -->
          <div class="space-y-2">
            <label class="text-base font-semibold text-pink-600">Email</label>
            <UInput
              v-model="email"
              type="email"
              size="lg"
              placeholder="you@example.com"
              color="primary"
              class="w-full rounded-xl"
              :ui="{
                base: 'bg-white text-pink-700 placeholder-pink-300',
                wrapper:
                  'focus-within:ring-2 focus-within:ring-pink-300 rounded-xl',
              }"
            />
          </div>

          <!-- Password -->
          <div class="space-y-2">
            <label class="text-base font-semibold text-pink-600"
              >Password</label
            >
            <UInput
              v-model="password"
              type="password"
              size="lg"
              placeholder="Create a password"
              color="primary"
              class="w-full rounded-xl"
              :ui="{
                base: 'bg-white text-pink-700 placeholder-pink-300',
                wrapper:
                  'focus-within:ring-2 focus-within:ring-pink-300 rounded-xl',
              }"
            />
          </div>

          <!-- Confirm -->
          <div class="space-y-2">
            <label class="text-base font-semibold text-pink-600"
              >Confirm Password</label
            >
            <UInput
              v-model="retypePassword"
              type="password"
              size="lg"
              placeholder="Confirm your password"
              color="primary"
              class="w-full rounded-xl"
              :ui="{
                base: 'bg-white text-pink-700 placeholder-pink-300',
                wrapper:
                  'focus-within:ring-2 focus-within:ring-pink-300 rounded-xl',
              }"
            />
          </div>

          <!-- Buttons -->
          <div class="pt-4 space-y-4">
            <UButton
              type="submit"
              block
              size="lg"
              color="primary"
              label="Create Account"
              class="w-full font-bold bg-pink-400 hover:bg-pink-300 text-white cursor-pointer"
            />
            <UButton
              @click="toLogin"
              variant="outline"
              block
              size="lg"
              color="primary"
              label="Login"
              class="w-full font-bold text-pink-600 border-pink-300 hover:bg-pink-50 cursor-pointer"
            />
          </div>
        </form>
      </UCard>
    </main>
  </div>
</template>

<script setup>
const username = ref("");
const email = ref("");
const password = ref("");
const retypePassword = ref("");

const router = useRouter();

const toLogin = () => router.push("/login");
const submitForm = () => {
  console.log("Register with:", {
    username: username.value,
    email: email.value,
    password: password.value,
  });

  useToast().add({
    description: "Account registration is currently disabled.",
    color: "error",
    title: "Oops!",
  });
};

useHead({
  title: "Register — Neko-Love",
  meta: [
    { name: "description", content: "Register for Neko-Love." },
    { name: "theme-color", content: "#ffbb88" },
  ],
});
</script>
