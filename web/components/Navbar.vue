<template>
  <header
    class="flex items-center justify-between px-6 py-4 border-b bg-zinc-900 border-zinc-700"
  >
    <NuxtLink to="/" class="flex items-center gap-2">
      <img
        src="/assets/logo.png"
        alt="Logo"
        class="h-10 w-10"
        draggable="false"
      />
      <h1 class="text-2xl font-extrabold tracking-wide">
        Neko-<span class="text-orange-200">Love</span>
      </h1>
    </NuxtLink>

    <div class="flex items-center gap-4">
      <UButton
        variant="ghost"
        size="lg"
        label="Discord Server"
        to="https://discord.gg/Az8RWJJ6fT"
        class="font-medium text-base cursor-pointer text-orange-200"
        target="_blank"
      />
      <UButton
        @click="toLogin"
        variant="ghost"
        size="lg"
        label="Login"
        class="font-medium text-base cursor-pointer text-orange-200"
      />
    </div>
  </header>
</template>

<script setup>
const toLogin = () => useRouter().push("/login");
</script>
