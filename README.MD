[![<basvandewiel>](https://circleci.com/gh/basvandewiel/doscontainer.svg?style=svg)](https://app.circleci.com/pipelines/github/basvandewiel/doscontainer) [![License](https://img.shields.io/badge/License-BSD_2--Clause-orange.svg)](https://opensource.org/licenses/BSD-2-Clause)

# DOS Container

Doscontainer is a toolbox that aims to make it easy to mass-generate large numbers of
virtual hard-disk image files and load those up with ready to run retro games and apps.

The primary focus of this project is the MiSTer FPGA project's AO486 core, with the possible
addition of the currently budding PCXT core at a later stage.

## The problem

Simply put? There are thousands of MS-DOS and early Windows games available, but the skill
to get these all running nicely is increasingly difficult to come by.

## The solution

Use principles from modern Infra-as-Code and DevOps to encode and preserve the skills of the
past so that future entusiasts won't need to overcome a large knowledge bump first before
enjoying those beautiful experiences for themselves.

## The nitty-gritty

The process looks simple enough from afar:

  - Generate virtual hard drive image for a particular retro game/app.
  - Install the required OS for it into the image.
  - Inject the actual game/app into the image.
  - Add drivers and other prerequisites into the mix.
  - Glue it together with a friendly bootup script.
  - Fun!

But the devil is in a million details along the way. This is a hobby project, so the time
investment is low and I'm learning a number of completely new skills myself so any help
is very much appreciated.

## Current progress

The main task that needs to be tackled, is a way to format the disk image with a valid
FAT12/16 filesystem that will boot MS-DOS. Whether this happens through the doscontainer
binary or some other tool isn't important. As progress currently stands, bootability of the
images is the main blocker.

## Contributing

Please use the GitHub issue tracking system. Pick an issue, fork the repo and send in the
pull requests. Do use the issue tracker, though, so it's visible that you're working on an
issue to prevent duplication of effort.
