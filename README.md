# ssh-sign

This is a smart ssh-keygen wrapper to improve the signing experience.

Let's take a look:

### Signing
```sh
# ssh-keygen
ssh-keygen -Y sign -f <path to signing key> -n file <path to file to sign>
# ssh-sign
ssh-sign -f <path to file to sign>
```
### Verification
```sh
# ssh-keygen
ssh-keygen -Y verify -f <path to allowed signers file> -I <identity> -n file -s <path to signature file> < <path to file to verify>
# ssh-sign
ssh-sign -v -f <path to file to sign>
```
## Description
The motivation behind the project was to sign and verify files with my ssh key. The original tool ssh-keygen requires quite a long list of parameters. So this wrapper was created which derives many of the parameters from the context or uses (for me) suitable defaults.

Optionally, most of the parameters can also be specified, but they are not mandatory.