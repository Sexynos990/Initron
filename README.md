
## initron

as of 2025 july 27th you probably shouldn’t be using initron (and honestly the name could be better)
this code is kind of a disaster compared to literally any other init
this was more of a “let me slap rust in your face and walk away” moment
but if for some reason you're here and want to touch this... pull requests are open
or just fork it like a normal person
just don’t expect clean commits or any sane logic holding this together

---

## how 2 install (if you must)

clone the repo like everyone else

```bash
git clone https://github.com/Sexynos990/Initron.git
cd initron
cargo build --release
```

drop the binary into `/sbin` like it’s something important

```bash
sudo install -Dm755 target/release/initron /sbin/initron
```

no idea why you'd trust it in `/sbin` but ok

---

## make it work? (i guess)

initron looks inside `/etc/initron.d` for anything executable
**if that folder isn’t there you’re going to get a panic and scream at me later**
you've been warned

```bash
sudo mkdir -p /etc/initron.d
```

throw whatever you want in there, example if you want NetworkManager to do... something

```bash
echo -e '#!/bin/sh\nexec /usr/bin/NetworkManager' | sudo tee /etc/initron.d/network
sudo chmod +x /etc/initron.d/network
```

if that even works you're luckier than most

---

## booting into this mess

you're on your own here but here’s the general idea

create a grub file
yes, one of those `/etc/grub.d/47_initron` type things 

```bash
sudo nano /etc/grub.d/47_initron
```

```bash
sudo chmod +x /etc/grub.d/47_initron
```

and **do not** just blindly copy this
you need to adapt it to whatever weird disk setup you have
also don't keep these example `set root` lines if you know they're wrong
grub will just laugh and drop you into a rescue shell

```
menuentry "linux + initron" {
    set root=(hd0,1)
    linux /vmlinuz-linux root=/dev/sdX1 init=/sbin/initron rw quiet
    initrd /initramfs-linux.img
}
```

then regenerate grub config so it can figure out what it hates

bios users:

```bash
sudo grub-mkconfig -o /boot/grub/grub.cfg
```

uefi users:

```bash
sudo grub-mkconfig -o /boot/efi/EFI/grub/grub.cfg
```

no promises this even gets picked up correctly

---

## done? maybe?

reboot
choose “linux + initron” from grub
and **if** your screen doesn’t instantly go black or hang at some vague kernel message
you’re now running an init system that probably shouldn’t exist
why? still no idea
but it *technically* boots

---

## why this exists (lol)

because memes
because you clicked the repo and somehow scrolled this far
there’s other init systems that are more mature, more stable, and maintained by people who actually care 
but hey you're here ...probably done the guide..
this one's yours now
