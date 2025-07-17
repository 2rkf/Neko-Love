<template>
  <header
    class="flex items-center justify-between px-6 py-4 bg-white/70 backdrop-blur border-b border-pink-200 shadow-md sticky top-0 z-50 animate-slide-in-top"
  >
    <NuxtLink to="/" class="flex items-center gap-2">
      <img
        src="/assets/logo.png"
        alt="Logo"
        class="h-10 w-10 animate-wiggle-slow"
        draggable="false"
      />
      <h1 class="text-2xl font-extrabold text-pink-600 tracking-wide">
        Ne<span class="text-orange-400">koi</span>
      </h1>
    </NuxtLink>

    <div class="flex items-center gap-4">
      <UButton
        variant="ghost"
        size="lg"
        label="Discord Server"
        to="https://discord.gg/Az8RWJJ6fT"
        class="font-medium text-base cursor-pointer text-orange-400 hover:scale-105 transition"
        target="_blank"
      />

      <div v-if="status == 'authenticated' && user">
        <UPopover :popper="{ placement: 'bottom-end' }" class="text-left">
          <UButton
            icon="i-lucide-user"
            variant="ghost"
            size="lg"
            class="text-orange-500 font-medium text-base cursor-pointer"
          >
            {{ user.nickname }}
          </UButton>
          <template #content>
            <div class="p-4 rounded-lg bg-white shadow-xl w-48 space-y-2">
              <NuxtLink to="/dashboard">
                <UButton
                  class="cursor-pointer"
                  icon="i-lucide-layout-grid"
                  label="Dashboard"
                  color="primary"
                  variant="ghost"
                  size="sm"
                  block
                />
              </NuxtLink>
              <UButton
                class="cursor-pointer"
                icon="i-lucide-log-out"
                label="Logout"
                color="error"
                variant="ghost"
                size="sm"
                block
                @click="logout"
              />
            </div>
          </template>
        </UPopover>
      </div>

      <div v-else-if="status == 'unauthenticated'">
        <UButton
          @click="toLogin"
          variant="ghost"
          size="lg"
          label="Login"
          class="font-medium text-base cursor-pointer text-orange-400 hover:scale-105 transition"
        />
      </div>
      <USkeleton v-else class="h-4 w-10" />
    </div>
  </header>
</template>

<script setup>
const toLogin = () => useRouter().push("/login");
const { logout, token, status, user } = useAuth();
</script>
