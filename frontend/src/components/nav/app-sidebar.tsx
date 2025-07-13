"use client";

import * as React from "react";
import {
  BookOpen,
  Bot,
  Frame,
  Map,
  PieChart,
  Settings2,
  SquareTerminal,
} from "lucide-react";

import { NavMain } from "@/components/nav/nav-main";
import { NavProjects } from "@/components/nav/nav-projects";
import { NavUser } from "@/components/nav/nav-user";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarRail,
} from "@/components/ui/sidebar";

// This is sample data.
const data = {
  user: {
    name: "kirito",
    email: "kirito@gmail.com",
    avatar: "/avatars/shadcn.jpg",
  },
  navMain: [
    {
      title: "Chronostasis",
      url: "#",
      icon: SquareTerminal,
      isActive: true,
      items: [
        {
          title: "Statistics",
          url: "#",
        },
        {
          title: "Invetory",
          url: "#",
        },
        {
          title: "Settings",
          url: "#",
        },
      ],
    },
    {
      title: "Uno",
      url: "#",
      icon: Bot,
      items: [
        {
          title: "Ipsum",
          url: "#",
        },
        {
          title: "Dolor",
          url: "#",
        },
        {
          title: "Sit",
          url: "#",
        },
      ],
    },
    {
      title: "Lorem",
      url: "#",
      icon: BookOpen,
      items: [
        {
          title: "Ipsum",
          url: "#",
        },
        {
          title: "Dolor",
          url: "#",
        },
        {
          title: "Sit",
          url: "#",
        },
        {
          title: "Amet",
          url: "#",
        },
      ],
    },
    {
      title: "Dos",
      url: "#",
      icon: Settings2,
      items: [
        {
          title: "Ipsum",
          url: "#",
        },
        {
          title: "Dolor",
          url: "#",
        },
        {
          title: "Sit",
          url: "#",
        },
        {
          title: "Amet",
          url: "#",
        },
      ],
    },
  ],
  projects: [
    {
      name: "Lorem",
      url: "#",
      icon: Frame,
    },
    {
      name: "Ipsum",
      url: "#",
      icon: PieChart,
    },
    {
      name: "Dolor",
      url: "#",
      icon: Map,
    },
  ],
};

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
  return (
    <Sidebar collapsible="icon" {...props}>
      <SidebarHeader>{/* <TeamSwitcher teams={data.teams} /> */}</SidebarHeader>
      <SidebarContent>
        <NavMain items={data.navMain} />
        <NavProjects projects={data.projects} />
      </SidebarContent>
      <SidebarFooter>
        <NavUser user={data.user} />
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  );
}
