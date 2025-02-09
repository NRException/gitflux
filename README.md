<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/nrexception/gitflux">
    <img src="images/banner.jpeg" alt="Logo" width="800" height="450">
  </a>

<h3 align="center">Gitflux</h3>

  <p align="center">
    Gitflux is an all in one conventional commit management and tag version manager.
    <br />
    <a href="https://github.com/nrexception/gitflux"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/nrexception/gitflux">View Demo</a>
    &middot;
    <a href="https://github.com/nrexception/gitflux/issues/new?labels=bug&template=bug-report---.md">Report Bug</a>
    &middot;
    <a href="https://github.com/nrexception/gitflux/issues/new?labels=enhancement&template=feature-request---.md">Request Feature</a>
  </p>

  <p align="center">
    <img alt="GitHub branch status" src="https://img.shields.io/github/checks-status/nrexception/gitflux/master">
    <img alt="GitHub branch status" src="https://img.shields.io/github/checks-status/nrexception/gitflux/develop">
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

Gitflux is a work in progress proof of concept to further deepen my understanding of Rust,
This tool is not intended for production usecases yet, but might be in the future. Please use at your own risk.

A lot of this README is still WIP so forgive any boilerplate still left over, thanks very much to othneildrew and his awesome readme template!

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->

## Usage

### Tag bumping

Gitflux provides full incremental management of your tags in your repo using simple semver formatting, any git tags that cannot be parsed to a simple semver format will be ignored. You can initialise a simple tag pointing to the HEAD of your repository using

```bash
gitflux bump --init
```

You can also increase logging level by parsing the `-v` flag, this can be used incrementally to increase the logging level, each 'v' representing a greater level of logging, for example:

```bash
gitflux bump --init -vvvv
```

This will provide the highest level of verbosity (information).

<img src="images/bumpinit.gif" alt="inittag" width="800" height="400">

Once you have initialised a semver tag, you can simply bump the patch level using

```bash
gitflux bump
```

<img src="images/bumptag.gif" alt="bumptag" width="800" height="400">

You can also pass the `--tag-schema major|minor|patch` or `-t major|minor|patch` flags to override the tag version that gitflux will bump your tag by:

```bash
gitflux bump --tag-schma minor
```

<img src="images/bumptagallvers.gif" alt="bumptagallvers" width="800" height="600">

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ROADMAP -->

## Roadmap

- [x] Get tag bumping functionality up and running.
- [x] Create development branch
- [ ] Tag releases from dev branch to master with gitflux
- [ ] Configure global settings serde parsing.
- [ ] CI/CD building for releases.
- [ ] Lock step tag version with build / release version? Rust is doable, other apps maybe?
- [ ] Implement conventional commit message manager and commit handling

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

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->

## License

Distributed under the GNU GENERAL PUBLIC LICENSE. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTACT -->

## Contact

Project Link: [https://github.com/nrexception/gitflux](https://github.com/nrexception/gitflux)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[contributors-shield]: https://img.shields.io/github/contributors/github_username/repo_name.svg?style=for-the-badge
[contributors-url]: https://github.com/github_username/repo_name/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/github_username/repo_name.svg?style=for-the-badge
[forks-url]: https://github.com/github_username/repo_name/network/members
[stars-shield]: https://img.shields.io/github/stars/github_username/repo_name.svg?style=for-the-badge
[stars-url]: https://github.com/github_username/repo_name/stargazers
[issues-shield]: https://img.shields.io/github/issues/github_username/repo_name.svg?style=for-the-badge
[issues-url]: https://github.com/github_username/repo_name/issues
[license-shield]: https://img.shields.io/github/license/github_username/repo_name.svg?style=for-the-badge
[license-url]: https://github.com/github_username/repo_name/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/linkedin_username
[product-screenshot]: images/screenshot.png
[Next.js]: https://img.shields.io/badge/next.js-000000?style=for-the-badge&logo=nextdotjs&logoColor=white
[Next-url]: https://nextjs.org/
[React.js]: https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB
[React-url]: https://reactjs.org/
[Vue.js]: https://img.shields.io/badge/Vue.js-35495E?style=for-the-badge&logo=vuedotjs&logoColor=4FC08D
[Vue-url]: https://vuejs.org/
[Angular.io]: https://img.shields.io/badge/Angular-DD0031?style=for-the-badge&logo=angular&logoColor=white
[Angular-url]: https://angular.io/
[Svelte.dev]: https://img.shields.io/badge/Svelte-4A4A55?style=for-the-badge&logo=svelte&logoColor=FF3E00
[Svelte-url]: https://svelte.dev/
[Laravel.com]: https://img.shields.io/badge/Laravel-FF2D20?style=for-the-badge&logo=laravel&logoColor=white
[Laravel-url]: https://laravel.com
[Bootstrap.com]: https://img.shields.io/badge/Bootstrap-563D7C?style=for-the-badge&logo=bootstrap&logoColor=white
[Bootstrap-url]: https://getbootstrap.com
[JQuery.com]: https://img.shields.io/badge/jQuery-0769AD?style=for-the-badge&logo=jquery&logoColor=white
[JQuery-url]: https://jquery.com
