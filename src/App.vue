<script lang="ts">
import { defineComponent } from "vue";
import Welcome from "@/components/Welcome.vue";

export default defineComponent({
  name: "App",
  components: {
    Welcome,
  },
});
</script>

<template>
    <Welcome />
</template>


<style>
/* Light mode */
:root {
  color-scheme: light dark; /* both supported */
  
  --body-bg: #fff;
  --body-color: #222;
}

/* Dark mode */
@media (prefers-color-scheme: dark) {
  :root {
    --body-bg: #121212;
    --body-color: #eee;
  }
}

body {
  background: var(--body-bg);
  color: var(--body-color);
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  margin-top: 60px;
}
</style>
