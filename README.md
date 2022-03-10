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
ssh-sign -v -f <path to file to verify>
```
## Description
The motivation behind the project was to sign and verify files with my ssh key. The original tool ssh-keygen requires quite a long list of parameters. So this wrapper was created which derives many of the parameters from the context or uses (for me) suitable defaults.

Optionally, most of the parameters can also be specified, but they are not mandatory.

The following defaults are used:

* signature file path => same as file path but with a ".sig" appended
* signature key path => ~/.ssh/file-sign.pub
* allowed signers path => ~/.ssh/allowed_signers

### Identity determination
Identity determination is enabled via the allowed signers file. The program iterates through the entered identities and stores them temporarily. After that, the process for verification is checked with each identity until one is successful. 

This procedure is necessary because there is no identity in the ssh key itself and therefore also not in the signature. 

If the identity is known in advance, it can be passed as a parameter and trying out all identities is skipped.
