import { Retcher, Browser } from './index';

// Set up the Retcher instance
const retcher = new Retcher({
    browser: Browser.Firefox,
    http3: true,
    ignoreTlsErrors: true,
});

(async () => {
    // Use the `fetch` method as you would with the built-in `fetch` function
    const response = await retcher.fetch("https://self-signed.badssl.com/");
    // const response = await retcher.fetch("https://www.karlin.mff.cuni.cz/~antoch/");
    
    console.log(response.text());
    console.log(response.headers);
})();
// console.log(await response.json());
// ..