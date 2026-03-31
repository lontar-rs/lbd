# 🤝 Contributing to Lontar Balinese Dictionary

Thank you for helping build the Lontar Balinese Dictionary! We welcome contributions from developers, linguists, and Balinese language enthusiasts.

## 🌱 Quick Start for Contributors

```bash
# 1. Fork and clone the repository
git clone https://github.com/YOUR-USERNAME/lontar-rs/lbd.git
cd lbd

# 2. Run the automated setup (installs everything!)
./scripts/setup.sh

# 3. Start development services
./scripts/start.sh

# 4. Open your browser and start contributing!
open http://localhost:5173
```

That's it! You now have a complete development environment with 45K+ Balinese words ready to work with.

---

## 🎯 Contribution Areas

### 🚀 **High Priority (Phase 4A)**
- **📝 Editorial Interface**: Build admin tools for dictionary curation
- **🔐 Authentication**: User login and role-based access
- **📋 Entry Management**: Draft → Review → Publish workflow
- **🔍 Bulk Operations**: Approve/reject multiple entries

### 🎨 **Frontend Improvements**
- **📱 Mobile Responsiveness**: Optimize for mobile devices
- **🌐 Internationalization**: Add more interface languages
- **♿ Accessibility**: WCAG compliance and screen reader support
- **🎨 UI/UX**: Improve user experience and visual design

### 🔧 **Backend Enhancements**
- **⚡ Performance**: Optimize search and database queries
- **🔍 Advanced Search**: Filters, faceted search, suggestions
- **📊 Analytics**: Usage statistics and search analytics
- **🔄 API Enhancements**: Additional endpoints and features

### 📚 **Content & Linguistics**
- **📖 Dictionary Entries**: Add more Balinese words and definitions
- **📜 Etymology**: Build etymological chains and language histories
- **🎭 Register System**:完善 unda-usuk (register) classifications
- **🌍 Dialect Support**: Add regional Balinese dialects

### 🛠️ **Infrastructure & DevOps**
- **🐳 Docker**: Containerize the application
- **🚀 Deployment**: CI/CD pipelines and production deployment
- **📝 Documentation**: Improve API docs and user guides
- **🧪 Testing**: Add unit tests, integration tests, E2E tests

---

## 🛠️ Development Workflow

### 1. Setup Environment
```bash
# Automated setup (recommended)
./scripts/setup.sh

# Or manual setup (see DEVELOPMENT.md)
```

### 2. Create Feature Branch
```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-description
```

### 3. Make Changes
- **Code**: Write your changes following the existing patterns
- **Test**: Ensure everything works correctly
- **Docs**: Update documentation if needed

### 4. Run Quality Checks
```bash
# Format code
cargo fmt

# Check for issues
cargo clippy

# Run tests
cargo test

# Run pre-commit hooks
pre-commit run --all-files
```

### 5. Submit Pull Request
```bash
git add .
git commit -m "feat: add your feature description"
git push origin feature/your-feature-name
# Then create PR on GitHub
```

---

## 📋 Development Guidelines

### 🎯 **Code Standards**
- **Rust**: Follow `rustfmt` and `clippy` recommendations
- **TypeScript/Svelte**: Use existing patterns and TypeScript strict mode
- **Database**: Use SQLx for type-safe queries
- **API**: Follow RESTful principles and document endpoints

### 🏗️ **Architecture Principles**
- **Modular**: Keep components focused and reusable
- **Type-safe**: Use Rust's type system and TypeScript
- **Testable**: Write tests for new functionality
- **Documented**: Add comments and documentation for complex code

### 🌱 **Balinese Language Guidelines**
- **Respect**: Be respectful of Balinese culture and language
- **Accuracy**: Ensure linguistic accuracy and proper register usage
- **Sources**: Cite sources for etymological claims
- **Community**: Consider community feedback on language usage

---

## 🔧 Technical Details

### **Stack Overview**
- **Backend**: Rust + Axum + SQLx + PostgreSQL
- **Frontend**: SvelteKit + TypeScript + TailwindCSS
- **Search**: Meilisearch
- **Database**: PostgreSQL with migrations

### **Service Management**
```bash
# Start all services
./scripts/start.sh

# Stop all services
./scripts/stop.sh

# Check status
./scripts/status.sh
```

### **Database Operations**
```bash
# Run migrations
sqlx migrate run

# Create new migration
sqlx migrate add description

# Access database
psql $DATABASE_URL
```

### **Import/Export**
```bash
# Import dictionary data
cargo run --bin import -- --file data/words.csv --source custom

# Update search index
./scripts/index_search.sh
```

---

## 🐛 Bug Reports

### **Reporting Issues**
1. **Check existing issues** first
2. **Use issue templates** when available
3. **Provide detailed information**:
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details
   - Screenshots if applicable

### **Bug Fix Process**
1. **Create issue** describing the bug
2. **Assign to yourself** (or ask for assignment)
3. **Create branch** with fix
4. **Add tests** to prevent regression
5. **Submit PR** with detailed description

---

## 💡 Feature Requests

### **Proposing Features**
1. **Check roadmap** in TODO.md first
2. **Create issue** with feature proposal
3. **Discuss implementation** approach
4. **Get community feedback**
5. **Start implementation** after approval

### **Feature Criteria**
- **Alignment**: Fits project goals and roadmap
- **Feasibility**: Technically achievable
- **Impact**: Benefits users and project
- **Maintenance**: Sustainable long-term

---

## 🔒 Security

### **Vulnerability Reporting**
- **Private disclosure**: Email maintainers privately
- **Public issues**: Use `SECURITY.md` guidelines
- **Responsibility**: Follow responsible disclosure

### **Security Practices**
- **Environment variables**: Use `.env` for credentials
- **Input validation**: Validate all user inputs
- **Dependencies**: Keep dependencies updated
- **Authentication**: Use secure authentication practices

---

## 📖 Documentation

### **Types of Documentation**
- **README.md**: Project overview and quick start
- **CONTRIBUTING.md**: Contribution guidelines (this file)
- **DEVELOPMENT.md**: Detailed development setup
- **API.md**: API documentation
- **ARCHITECTURE.md**: System architecture
- **TODO.md**: Development roadmap

### **Writing Documentation**
- **Clear**: Use clear, concise language
- **Complete**: Include all necessary information
- **Current**: Keep documentation up to date
- **Examples**: Provide code examples and usage

---

## 🤝 Community Guidelines

### **Code of Conduct**
- **Respectful**: Be respectful to all community members
- **Inclusive**: Welcome contributors of all backgrounds
- **Constructive**: Provide constructive feedback
- **Helpful**: Help others learn and contribute

### **Communication Channels**
- **GitHub Issues**: Bug reports and feature requests
- **Discussions**: General questions and discussions
- **PR Reviews**: Code review and feedback

---

## 🎉 Recognition

### **Contributor Recognition**
- **Credits**: Contributors listed in README
- **Release Notes**: Mentioned in release announcements
- **Community**: Recognition in community channels

### **Types of Contributions**
- **Code**: Features, fixes, improvements
- **Documentation**: Docs, guides, tutorials
- **Design**: UI/UX, graphics, branding
- **Community**: Support, feedback, testing
- **Linguistic**: Language expertise, content

---

## 🚀 Getting Help

### **Resources**
- **README.md**: Project overview and setup
- **DEVELOPMENT.md**: Detailed development guide
- **TODO.md**: Current roadmap and tasks
- **Issues**: Open questions and discussions
- **Discussions**: Community conversations

### **Contact**
- **GitHub Issues**: For bugs and feature requests
- **Discussions**: For general questions
- **Maintainers**: For specific project inquiries

---

## 📜 License

By contributing to LBD, you agree that your contributions will be licensed under the same license as the project (Apache-2.0 for software, CC BY-SA 4.0 for dictionary data).

---

**Thank you for contributing to the preservation and celebration of the Balinese language! 🌟**

*ᬮᭀᬦ᭄ᬢᬭᬄ ᬩᬲ ᬩᬮᬮᬶ — Lontar Basa Bali*
