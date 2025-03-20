fn main() {
    let svg = r###"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg width="600" height="150" viewBox="0 0 600 150" xmlns="http://www.w3.org/2000/svg">
    <defs>
        <linearGradient id="steelBase" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" stop-color="#7a7a7a"/>
            <stop offset="20%" stop-color="#9a9a9a"/>
            <stop offset="40%" stop-color="#888888"/>
            <stop offset="60%" stop-color="#a6a6a6"/>
            <stop offset="80%" stop-color="#858585"/>
            <stop offset="100%" stop-color="#919191"/>
        </linearGradient>
        <filter id="roughTexture">
            <feTurbulence type="fractalNoise" baseFrequency="0.15" numOctaves="5" seed="3" result="noise"/>
            <feDisplacementMap in="SourceGraphic" in2="noise" scale="3" xChannelSelector="R" yChannelSelector="G" result="displacedNoise"/>
            <feComposite operator="arithmetic" k1="1" k2="0" k3="0" k4="0" in="displacedNoise" in2="SourceGraphic"/>
        </filter>
        <filter id="scratches">
            <feTurbulence type="turbulence" baseFrequency="0.05" numOctaves="2" seed="5" result="turb"/>
            <feDisplacementMap in="SourceGraphic" in2="turb" scale="1" xChannelSelector="R" yChannelSelector="G"/>
        </filter>
        <filter id="lighting">
            <feDiffuseLighting in="SourceGraphic" surfaceScale="2" diffuseConstant="1" result="diffLight">
                <feDistantLight azimuth="240" elevation="45"/>
            </feDiffuseLighting>
            <feComposite in="SourceGraphic" in2="diffLight" operator="arithmetic" k1="1" k2="0" k3="0.3" k4="0"/>
        </filter>
    </defs>
    <rect width="600" height="150" fill="white"/>
    <text x="300" y="100" text-anchor="middle" font-family="Arial" font-size="100" font-weight="900">
        <tspan fill="url(#steelBase)" stroke="black" stroke-width="1" filter="url(#roughTexture) url(#scratches) url(#lighting)">FarraOx</tspan>
    </text>
</svg>"###;

    std::fs::write("examples/basic-website/assets/img/logo.svg", svg).unwrap();
}