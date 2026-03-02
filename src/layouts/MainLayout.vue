<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated class="glass border-radius-all q-ma-md">
      <q-toolbar>
        <q-btn flat dense round icon="menu" aria-label="Menu" @click="toggleLeftDrawer" />
      </q-toolbar>
    </q-header>

    <q-drawer class="glass-drawer" v-model="leftDrawerOpen">
      <q-list>
        <q-item-label header>Navagation </q-item-label>
        <q-item clickable v-ripple :to="{ name: 'main' }">
          <q-item-section>Fabrics</q-item-section>
        </q-item>
      </q-list>
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const leftDrawerOpen = ref(false);

function toggleLeftDrawer() {
  leftDrawerOpen.value = !leftDrawerOpen.value;
}
</script>
