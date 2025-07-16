<template>
  <div class="min-h-screen flex flex-col bg-gradient-to-b from-pink-100 via-pink-50 to-rose-100 font-[Nunito] noselect">
    <Navbar />

    <main class="flex-grow flex items-center justify-center px-4 py-16">
      <UCard
        class="w-full max-w-md shadow-xl border-pink-200 bg-white/80 backdrop-blur animate-fade-in-up"
        :ui="{ body: 'p-8 space-y-6 text-left' }"
      >
        <!-- Header -->
        <div class="text-center space-y-2">
          <h2 class="text-4xl font-extrabold text-pink-600 drop-shadow-pink">Login</h2>
          <p class="text-pink-700">Please sign in to continue</p>
        </div>

        <!-- Form -->
        <form class="space-y-4">
          <!-- Username -->
          <div class="space-y-2">
            <label class="text-base font-semibold text-pink-600">Username</label>
            <UInput
              v-model="username"
              size="lg"
              placeholder="Enter your username"
              color="primary"
              class="w-full rounded-xl"
              :ui="{
                base: 'bg-white text-pink-700 placeholder-pink-300',
                wrapper: 'focus-within:ring-2 focus-within:ring-pink-300 rounded-xl',
              }"
            />
          </div>

          <!-- Password -->
          <div class="space-y-2">
            <label class="text-base font-semibold text-pink-600">Password</label>
            <UInput
              v-model="password"
              type="password"
              size="lg"
              placeholder="Enter your password"
              color="primary"
              class="w-full rounded-xl"
              :ui="{
                base: 'bg-white text-pink-700 placeholder-pink-300',
                wrapper: 'focus-within:ring-2 focus-within:ring-pink-300 rounded-xl',
              }"
            />
          </div>

          <!-- Buttons -->
          <div class="pt-4 space-y-4">
            <UButton
              @click.prevent="authorise"
              block
              size="lg"
              color="primary"
              label="Login"
              class="w-full font-bold bg-pink-400 hover:bg-pink-300 text-white cursor-pointer"
            />
            <UButton
              @click="toRegister"
              variant="outline"
              block
              size="lg"
              color="primary"
              label="Create an Account"
              class="w-full font-bold text-pink-600 border-pink-300 hover:bg-pink-50 cursor-pointer"
            />
          </div>
        </form>
      </UCard>
    </main>
  </div>
</template>

<script setup>
const config = useAppConfig();
const username = ref("");
const password = ref("");
const toRegister = () => useRouter().push("/register");
const toast = useToast();
const { login } = useAuth();

async function authorise() {
  try {
    const { data, error } = await useFetch(`${config.API_URL}/auth/authorise`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${config.API_KEY}`,
      },
      body: JSON.stringify({
        username: username.value,
        password: password.value,
      }),
    });

    if (error.value) {
      toast.add({
        title: "Login Error",
        description: "Incorrect username or password.",
        color: "error",
      });
    } else if (data.value) {
      login(data.value.message);
      navigateTo("/dashboard");
    } else {
      toast.add({
        title: "Unexpected Error",
        description: "Please try again later.",
        color: "warning",
      });
    }
  } catch (err) {
    console.error(err);
    toast.add({
      title: "Server Error",
      description: "Could not connect to the server.",
      color: "error",
    });
  }
}

useHead({
  title: "Login — Neko-Love",
  meta: [
    { name: "description", content: "Login to Neko-Love." },
    { name: "theme-color", content: "#ffd8e0" },
  ],
});
</script>
