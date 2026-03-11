import { createRouter, createWebHistory } from "vue-router";

import NoteListPage from "../pages/notes/NoteListPage.vue";
import NoteEditPage from "../pages/notes/NoteEditPage.vue";
import TagListPage from "../pages/tags/TagListPage.vue";
import CollectionListPage from "../pages/collections/CollectionListPage.vue";
import CollectionEditPage from "../pages/collections/CollectionEditPage.vue";
import CollectionResultsPage from "../pages/collections/CollectionResultsPage.vue";

const routes = [
  { path: "/", name: "NoteList", component: NoteListPage },
  { path: "/notes/new", name: "NoteCreate", component: NoteEditPage },
  { path: "/notes/:id", name: "NoteEdit", component: NoteEditPage },
  { path: "/tags", name: "TagList", component: TagListPage },
  { path: "/collections", name: "CollectionList", component: CollectionListPage },
  { path: "/collections/new", name: "CollectionCreate", component: CollectionEditPage },
  { path: "/collections/:id/edit", name: "CollectionEdit", component: CollectionEditPage },
  { path: "/collections/:id/results", name: "CollectionResults", component: CollectionResultsPage },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
