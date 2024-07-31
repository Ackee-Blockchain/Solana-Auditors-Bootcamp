
![Solana Auditors Bootcamp](.banner/banner.png)

# Solana Auditors Bootcamp

Learn to audit Solana programs and help secure the ecosystem. Take your security practices to the next level and get certified by Ackee Blockchain Security. It's free, too.

Solana-Auditors-Bootcamp Season 1:
- **100% free**
- **7 weeks**, **online** & **in English**
- Starting on **Aug 14**
- Supported by **Solana Foundation**


# What can I expect during the course?

|Week #|Lecture|
|--|--|
|Week 1|[**Advanced Anchor**](./Lesson-1/README.md)|
|Week 2|[**Integration Tests and Unit Tests**](./Lesson-2/README.md)|
|Week 3|[**Fuzzing with Trident I**](./Lesson-3/README.md)|
|Week 4|[**Fuzzing with Trident II**](./Lesson-4/README.md)|
|Week 5|[**Security Best Practices**](./Lesson-5/README.md)|
|Week 6|[**Common Vulnerability Vectors**](./Lesson-6/README.md)|
|Week 7|**Graduation Ceremony**|
|-|-|
|Challenge|[**Capture the Flag**](./Capture-the-Flag/README.md)|
</div>

-----

# What to prepare:

[WSL]: https://learn.microsoft.com/en-us/windows/wsl/install
[Solana]: https://docs.solanalabs.com/cli/install
[Anchor]: https://www.anchor-lang.com/docs/installation

## Manual Setup

| Setup                                      | Description                     | Version               | How to Install                |
| -------------------------------------------| --------------------------------| ----------------------| ------------------------------|
| Windows subsystem for Linux(WSL)           | optional but highly recommended | 2.0                   | [Instructions][WSL]           |
| Solana tool suite                          | -                               | 1.18.18               | [Instructions][Solana]        |
| Anchor framework                           | -                               | 0.30.1                | [Instructions][Anchor]        |

## Docker Image
We also prepared Docker Image for you to use with all required dependencies.

To use the pre-built Docker image for this course, you can **PULL IT** from **Docker Hub:**

```bash
docker pull your-dockerhub-username/your-repo-name:tag
```

**Once pulled, RUN:**
```bash
docker run -it -p 8899:8899 -p 9900:9900 -p 8000:8000 -p 8080:8080 your-dockerhub-username/your-repo-name:tag
```

**Once started, visit the URL:**
```url
http://localhost:8080/
```

-----

# How to participate:

- You can **participate** by sending an application: [Solana Auditors Bootcamp](https://ackee.xyz/solana-auditors-bootcamp)

-----
