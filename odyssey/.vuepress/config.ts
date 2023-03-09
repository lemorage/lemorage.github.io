import { viteBundler } from "@vuepress/bundler-vite";
import { defineUserConfig } from "vuepress";
import { Page } from "@vuepress/core";
import { searchPlugin } from "@vuepress/plugin-search";
import { gungnirTheme } from "vuepress-theme-gungnir";

const isProd = process.env.NODE_ENV === "production";

export default defineUserConfig({
  title: "Lemorage",
  description: "Oh My World!",

  head: [
    [
      "link",
      {
        rel: "icon",
        type: "image/png",
        sizes: "16x16",
        href: `/img/logo/favicon-16x16.png`
      }
    ],
    [
      "link",
      {
        rel: "icon",
        type: "image/png",
        sizes: "32x32",
        href: `/img/logo/favicon-32x32.png`
      }
    ],
    ["meta", { name: "application-name", content: "Lemorage" }],
    ["meta", { name: "apple-mobile-web-app-title", content: "Lemorage" }],
    [
      "meta",
      { name: "apple-mobile-web-app-status-bar-style", content: "black" }
    ],
    [
      "link",
      { rel: "apple-touch-icon", href: `/img/logo/apple-touch-icon.png` }
    ],
    ["meta", { name: "theme-color", content: "#377bb5" }],
    ["meta", { name: "msapplication-TileColor", content: "#377bb5" }]
  ],

  bundler: viteBundler(),

  theme: gungnirTheme({
    repo: "lemorage/lemorage.github.io",
    docsDir: "odyssey",
    docsBranch: "master",

    hitokoto: "https://v1.hitokoto.cn?c=b&i=c",

    // personal information
    personalInfo: {
      name: "Lemorage",
      avatar: "https://www.gravatar.com/avatar/8eb73bbf5c825a52672914fa1959388d?d=identicon&s=256",
      description: "Those with a lack of purpose fill the void with pleasure.",
      sns: {
        github: "lemorage",
        linkedin: "miao-z-40059721b",
        twitter: "LemorageOne",
        email: "one.lemorage@gmail.com",
        rss: "/rss.xml"
      }
    },

    // header images on home page
    homeHeaderImages: [
      {
        path: "/img/home-bg/1.png",
        mask: "rgba(40, 57, 101, .4)"
      },
      {
        path: "/img/home-bg/2.jpg",
        mask: "rgba(196, 176, 131, .1)"
      },
      {
        path: "/img/home-bg/3.jpg",
        mask: "rgba(68, 74, 83, .1)"
      },
      {
        path: "/img/home-bg/4.png",
        mask: "rgba(19, 75, 50, .2)"
      },
      {
        path: "/img/home-bg/5.jpg"
      }
    ],

    // other pages
    pages: {
      tags: {
        title:
          "Logs",
        subtitle:
          "¯\\_(ツ)_/¯",
        bgImage: {
          path: "/img/pages/tags.jpg",
          mask: "rgba(225, 172, 81, .5)"
        }
      },
      links: {
        subtitle:
          "Culture Club",
        bgImage: {
          path: "/img/pages/links.jpg",
          mask: "rgba(154, 198, 220, .2)"
        }
      }
    },

    themePlugins: {
      // enable plugins
      search: false,
      git: isProd,
      katex: true,
      chartjs: {
        token: "chart"
      },
      giscus: {
        repo: "This-is-an-Apple/blog-giscus-comments",
        repoId: "R_kgDOGl2SjQ",
        category: "Announcements",
        categoryId: "DIC_kwDOGl2Sjc4CAcxK",
        darkTheme: "https://lemorage.github.io/styles/giscus-dark.css"
      },
      mdPlus: {
        all: true
      },
      ga: "G-HCQSX53XFG",
      ba: "75381d210789d3eaf855fa16246860cc",
      rss: {
        siteURL: "https://lemorage.github.io",
        copyright: "© 2023 Lemorage"
      }
    },

    navbar: [
      {
        text: "Home",
        link: "/",
        icon: "fa-fort-awesome"
      },
      {
        text: "Logs",
        link: "/tags/",
        icon: "fa-tag"
      },
      {
        text: "Links",
        link: "/link/",
        icon: "fa-satellite-dish"
      },
      {
        text: "About",
        link: "/about/",
        icon: "fa-cocktail"
      },
      {
        text: "Portfolio",
        link: "https://lemorage.github.io/",
        icon: "oi-rocket"
      }
    ],

    footer: `
      &copy; <a href="https://github.com/lemorage" target="_blank">Lemorage</a> 2023
      <br>
      Powered by <a href="https://v2.vuepress.vuejs.org" target="_blank">VuePress</a> &
      <a href="https://github.com/Renovamen/vuepress-theme-gungnir" target="_blank">Gungnir</a>
    `
  }),
  markdown: {
    headers: {
      level: [2, 3, 4, 5]
    },
    code: {
      lineNumbers: true
    }
  },
  plugins: [
    searchPlugin({
      locales: {
        "/": {
          placeholder: "Search",
        }
      },
      maxSuggestions: 7,
      isSearchable: (page: Page) => !["Logs", "Links", "Home"].includes(page.frontmatter.layout ? page.frontmatter.layout : "layout")
    })
  ]
});
