import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '@/views/HomeView.vue'
import PrizesView from '@/views/PrizesView.vue'
import QuestionView from '@/views/QuestionView.vue'
import ControllerView from '@/views/ControllerView.vue'
import PanelView from '@/views/PanelView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    {
      path: '/prize',
      name: 'prize',
      component: PrizesView,
    },
    {
      path: '/question',
      name: 'question',
      component: QuestionView
    },
    {
      path: '/controller',
      name: 'controller',
      component: ControllerView
    },
    {
      path: '/panel',
      name: 'panel',
      component: PanelView
    }
  ],
})

export default router
