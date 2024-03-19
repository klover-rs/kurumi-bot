const puppeteer = require('puppeteer');
const path = require('path');
const fs = require('fs');

(async () => {

  const argBase64 = process.argv[2];
  

  const argString = Buffer.from(argBase64, 'base64').toString('utf-8');
  
  const argObject = JSON.parse(argString);

  const browser = await puppeteer.launch();
  const page = await browser.newPage();

  await page.goto('file:///' + argObject.path);

  await page.waitForSelector('span');

  await page.waitForSelector('span');

  // Get the bounding box of the span element
  const spanBoundingBox = await page.evaluate(() => {
    const span = document.querySelector('body > code');
    const { x, y, width, height } = span.getBoundingClientRect();
    return { x, y, width, height };
  });

  await page.setViewport({
    width: argObject.width, 
    height: argObject.height, 
    deviceScaleFactor: 1, 
  });


  let image_path = path.join(__dirname, generateRandomString(10) + '.png');
 


  await page.screenshot({
    path: image_path,
    clip: {
      x: spanBoundingBox.x,
      y: spanBoundingBox.y,
      width: spanBoundingBox.width,
      height: spanBoundingBox.height / 2.5
    }
  });

  console.log(image_path);

  await browser.close();


})();

function generateRandomString(length) {
  const charset = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  let randomString = '';
  for (let i = 0; i < length; i++) {
    const randomIndex = Math.floor(Math.random() * charset.length);
    randomString += charset[randomIndex];
  }
  return randomString;
}