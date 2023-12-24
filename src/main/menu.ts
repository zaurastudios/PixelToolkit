import { Menu, BrowserWindow } from "electron";

export default class MenuBuilder {
  mainWindow: BrowserWindow;

  constructor(mainWindow: BrowserWindow) {
    this.mainWindow = mainWindow;
  }

  buildMenu(): Menu {
    if (
      process.env.NODE_ENV === "development" ||
      process.env.DEBUG_PROD === "true"
    ) {
      this.setupDevelopmentEnvironment();
    }

    const template = this.buildDefaultTemplate();

    const menu = Menu.buildFromTemplate(template);
    Menu.setApplicationMenu(menu);

    return menu;
  }

  setupDevelopmentEnvironment(): void {
    this.mainWindow.webContents.on("context-menu", (_, props) => {
      const { x, y } = props;

      Menu.buildFromTemplate([
        {
          label: "Inspect element",
          click: () => {
            this.mainWindow.webContents.inspectElement(x, y);
          },
        },
      ]).popup({ window: this.mainWindow });
    });
  }

  buildDefaultTemplate() {
    const isMac = process.platform === "darwin";
    const templateDefault = [
      {
        label: "&File",
        submenu: [
          {
            label: "&Home",
            accelerator: isMac ? "Cmd+H" : "Ctrl+H",
            click: () => this.mainWindow.webContents.send("go-home"),
          },
          {
            label: "&Quit",
            accelerator: isMac ? "Cmd+Q" : "Ctrl+Q",
            click: () => {
              this.mainWindow.close();
            },
          },
        ],
      },
      {
        label: "&View",
        submenu:
          process.env.NODE_ENV === "development" ||
          process.env.DEBUG_PROD === "true"
            ? [
                {
                  label: "&Reload",
                  accelerator: "Ctrl+R",
                  click: () => {
                    this.mainWindow.webContents.reload();
                  },
                },
                {
                  label: "Toggle &Full Screen",
                  accelerator: "F11",
                  click: () =>
                    this.mainWindow.setFullScreen(
                      !this.mainWindow.isFullScreen(),
                    ),
                },
                {
                  label: "Toggle &Developer Tools",
                  accelerator: "Alt+Ctrl+I",
                  click: () => {
                    this.mainWindow.webContents.toggleDevTools();
                  },
                },
                {
                  label: "Theme",
                  submenu: [
                    {
                      label: "Dark",
                      click: () =>
                        this.mainWindow.webContents.send("change-theme", {
                          theme: "dark",
                        }),
                    },
                    {
                      label: "Light",
                      click: () =>
                        this.mainWindow.webContents.send("change-theme", {
                          theme: "light",
                        }),
                    },
                  ],
                },
              ]
            : [
                {
                  label: "Toggle &Full Screen",
                  accelerator: isMac ? "Cmd+Shift+F" : "F11",
                  click: () => {
                    this.mainWindow.setFullScreen(
                      !this.mainWindow.isFullScreen(),
                    );
                  },
                },
                {
                  label: "Theme",
                  submenu: [
                    {
                      label: "Dark",
                      click: () =>
                        this.mainWindow.webContents.send("change-theme", {
                          theme: "dark",
                        }),
                    },
                    {
                      label: "Light",
                      click: () =>
                        this.mainWindow.webContents.send("change-theme", {
                          theme: "light",
                        }),
                    },
                  ],
                },
              ],
      },
    ];

    return templateDefault;
  }
}
