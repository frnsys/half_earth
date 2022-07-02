// main.js

// Modules to control application life and create native browser window
const { app, protocol, BrowserWindow } = require('electron')
const path = require('path')

const createWindow = () => {
  // Create the browser window.
  const mainWindow = new BrowserWindow({
    width: 1280,
    height: 720,
    autoHideMenuBar: true,
  })
  mainWindow.maximize();

  // and load the index.html of the app.
  mainWindow.loadFile('index.html');

  // Open the DevTools.
  // mainWindow.webContents.openDevTools()
}

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app.whenReady().then(() => {
  // Change relative paths to absolute paths
  protocol.interceptFileProtocol('file', function(req, callback) {
    if (process.platform === 'win32') {
      // Assume if this substring is present
      // we're already dealing with an absolute path
      if (req.url.includes('resources/app')) {
        // Strip `file:///`
        let path = req.url.substr(8);
        callback({path: decodeURI(path)});
      } else {
        // Strip `file:///E:`
        var url = req.url.substr(10);
        if (url.startsWith('/assets')) {
          const p = path.normalize(__dirname + url);
          callback({path: decodeURI(p)})
        }
      }
    } else {
      // Strip off `file://`
      const url = req.url.substr(7);
      const p = url.startsWith('/assets') ? path.normalize(__dirname + url) : url;
      // Decoding seems necessary to avoid issues with e.g. spaces in path?
      callback({path: decodeURI(p)})
    }
  })

  createWindow()

  app.on('activate', () => {
    // On macOS it's common to re-create a window in the app when the
    // dock icon is clicked and there are no other windows open.
    if (BrowserWindow.getAllWindows().length === 0) createWindow()
  })
})

// Quit when all windows are closed, except on macOS. There, it's common
// for applications and their menu bar to stay active until the user quits
// explicitly with Cmd + Q.
app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') app.quit()
})

// In this file you can include the rest of your app's specific main process
// code. You can also put them in separate files and require them here.