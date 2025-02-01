/* eslint-disable */

// @ts-nocheck

// noinspection JSUnusedGlobalSymbols

// This file was automatically generated by TanStack Router.
// You should NOT make any changes in this file as it will be overwritten.
// Additionally, you should also exclude this file from your linter and/or formatter to prevent it from being checked or modified.

// Import Routes

import { Route as rootRoute } from "./routes/__root";
import { Route as RemoteImport } from "./routes/remote";
import { Route as AboutImport } from "./routes/about";
import { Route as IndexImport } from "./routes/index";

// Create/Update Routes

const RemoteRoute = RemoteImport.update({
    id: "/remote",
    path: "/remote",
    getParentRoute: () => rootRoute,
} as any);

const AboutRoute = AboutImport.update({
    id: "/about",
    path: "/about",
    getParentRoute: () => rootRoute,
} as any);

const IndexRoute = IndexImport.update({
    id: "/",
    path: "/",
    getParentRoute: () => rootRoute,
} as any);

// Populate the FileRoutesByPath interface

declare module "@tanstack/react-router" {
    interface FileRoutesByPath {
        "/": {
            id: "/";
            path: "/";
            fullPath: "/";
            preLoaderRoute: typeof IndexImport;
            parentRoute: typeof rootRoute;
        };
        "/about": {
            id: "/about";
            path: "/about";
            fullPath: "/about";
            preLoaderRoute: typeof AboutImport;
            parentRoute: typeof rootRoute;
        };
        "/remote": {
            id: "/remote";
            path: "/remote";
            fullPath: "/remote";
            preLoaderRoute: typeof RemoteImport;
            parentRoute: typeof rootRoute;
        };
    }
}

// Create and export the route tree

export interface FileRoutesByFullPath {
    "/": typeof IndexRoute;
    "/about": typeof AboutRoute;
    "/remote": typeof RemoteRoute;
}

export interface FileRoutesByTo {
    "/": typeof IndexRoute;
    "/about": typeof AboutRoute;
    "/remote": typeof RemoteRoute;
}

export interface FileRoutesById {
    __root__: typeof rootRoute;
    "/": typeof IndexRoute;
    "/about": typeof AboutRoute;
    "/remote": typeof RemoteRoute;
}

export interface FileRouteTypes {
    fileRoutesByFullPath: FileRoutesByFullPath;
    fullPaths: "/" | "/about" | "/remote";
    fileRoutesByTo: FileRoutesByTo;
    to: "/" | "/about" | "/remote";
    id: "__root__" | "/" | "/about" | "/remote";
    fileRoutesById: FileRoutesById;
}

export interface RootRouteChildren {
    IndexRoute: typeof IndexRoute;
    AboutRoute: typeof AboutRoute;
    RemoteRoute: typeof RemoteRoute;
}

const rootRouteChildren: RootRouteChildren = {
    IndexRoute: IndexRoute,
    AboutRoute: AboutRoute,
    RemoteRoute: RemoteRoute,
};

export const routeTree = rootRoute
    ._addFileChildren(rootRouteChildren)
    ._addFileTypes<FileRouteTypes>();

/* ROUTE_MANIFEST_START
{
  "routes": {
    "__root__": {
      "filePath": "__root.tsx",
      "children": [
        "/",
        "/about",
        "/remote"
      ]
    },
    "/": {
      "filePath": "index.tsx"
    },
    "/about": {
      "filePath": "about.tsx"
    },
    "/remote": {
      "filePath": "remote.tsx"
    }
  }
}
ROUTE_MANIFEST_END */
