<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a id="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->


<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/Big-Smarty/nvidia_oc_daemon">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">The NVIDIA_OC systemd daemon</h3>

  <p align="center">
    An awesome tool to tweak your NVIDIA GPU regardless of your graphics platform! 
    <br />
    <!-- <a href="https://github.com/othneildrew/Best-README-Template"><strong>Explore the docs »</strong></a> -->
    <br />
    <br />
    <!--<a href="https://github.com/othneildrew/Best-README-Template">View Demo</a> -->
    ·
    <!-- <a href="https://github.com/othneildrew/Best-README-Template/issues/new?labels=bug&template=bug-report---.md">Report Bug</a>-->
    ·
    <!-- <a href="https://github.com/othneildrew/Best-README-Template/issues/new?labels=enhancement&template=feature-request---.md">Request Feature</a>-->
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

[![Product Name Screen Shot][product-screenshot]](https://example.com)
GreenWithEnvy is a great tool to tweak your NVIDIA GPU. Unfortunately, as it works with X.Org coolbits, it doesn't work on wayland (at all). An alternative to just set your GPU's power settings would be the official nvidia-settings, but, as you might have guessed, it also only works on X.Org.

Here's why you should use NVIDIA_OC:
* You paid for the whole speedometer, so you're gonna use the whole spee- no wait. You paid for your whole GPU, so you should be able to use all of your GPU and not be limited by your graphics platform of choice.
* YEARS of using NVIDIA GPUs with default settings on wayland, but NO REAL WORLD USE found to leave potential performance untapped!
* You should LOVE yourself NOW and finally get those 5% more FPS out of your device! :smile:

Of course, no program is perfect, and I can't add failsaves for all possible edge cases for all possible GPUs, so you're on your own risk wise.
!! I TAKE NO RESPONSIBILITY FOR BRICKED DEVICES!!
I would assume that overclocking your GPU via programs using NVML (which this project utilizes) would void your warranty, and you guessed it: I won't take any responsibility for that either.

!!I REPEAT: I TAKE NO RESPONSIBILITY FOR ANYTHING THAT HAPPENED BECAUSE YOU USED THIS PROGRAM!! YOU USED THIS ON YOUR OWN BEHALF AND SHOULD TAKE ANY REPERCUSSIONS BY YOURSELF AS WELL!!

<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With

This section should list any major frameworks/libraries used to bootstrap your project. Leave any add-ons/plugins for the acknowledgements section. Here are a few examples.

* [log][log-url]
* [simplelog][simplelog-url]
* [serde][serde-url]
* [toml][toml-url]
* [notify][notify-url]
* [nvml][nvml-url]
* [nvml-wrapper][nvml-wrapper-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

TODO

### Prerequisites
TODO

### Installation

TODO

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage
TODO

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

- [x] create daemon
- [x] load config file
- [x] apply config
- [ ] Add GUI
- [ ] build time machine
- [ ] Multi-language Support
    - [ ] Chinese
    - [ ] Spanish
    - [ ] French (this will NEVER be implemented for I HATE the french language)

See the [open issues](https://github.com/Big-Smarty/nvidia_oc_daemon/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Top contributors:

<a href="https://github.com/Big-Smarty/nvidia_oc_daemon/graphs/contributors">
  <p>Contributors</p>
</a>

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Big Smarty - [@youre_never_getting_my_twitter](https://twitter.com/) - my@privacy.com

Project Link: [https://github.com/Big-Smarty/nvidia_oc_daemon](https://github.com/Big-Smarty/nvidia_oc_daemon)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments
I've ~~stolen~~ been inspired by the following sources:

* [Dreaming-Codes CLI nvidia overclocking tool](https://github.com/Dreaming-Codes/nvidia_oc/)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[nvml-url]: https://developer.nvidia.com/management-library-nvml
[nvml-wrapper-url]:https://github.com/Cldfire/nvml-wrapper
[rust-url]: https://www.rust-lang.org/
[log-url]: https://github.com/rust-lang/log
[simplelog-url]: https://github.com/baoyachi/simple-log
[serde-url]: https://github.com/serde-rs/serde
[toml-url]: https://github.com/toml-rs/toml
[notify-url]: https://github.com/notify-rs/notify.git
