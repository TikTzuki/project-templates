# FEAT-XXX: [Feature Name]

**Status:** DRAFT | ACTIVE | PAUSED | DONE | BLOCKED

> **Status Guide:**
> - DRAFT: Đang planning/design
> - ACTIVE: Đang implement
> - PAUSED: Tạm dừng (xem Checkpoint bên dưới)
> - DONE: Hoàn thành
> - BLOCKED: Bị block bởi dependency

---

## 1. Proposal

### Problem Statement

[Vấn đề cần giải quyết là gì?]

### Proposed Solution

[Mô tả giải pháp đề xuất]

### User Stories

- As a [user type], I want [goal] so that [benefit]

### Requirements

| ID        | Requirement   | Priority        |
|-----------|---------------|-----------------|
| FR-XXX-01 | [Description] | High/Medium/Low |

### Alternatives Considered

| Option     | Pros   | Cons   |
|------------|--------|--------|
| [Option A] | [Pros] | [Cons] |

---

## 2. Wireframes

> Bỏ qua section này nếu không có UI

### Screen: [Name]

```
┌─────────────────────────────────────────────────────────────────┐
│  [Screen Title]                                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  [ASCII wireframe here]                                         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Components used:** [List from _COMPONENTS.md]

---

## 3. Code Analysis

> **AI: PHẢI fill section này TRƯỚC khi implement**

### Related Files

| File               | Purpose        | Impact                |
|--------------------|----------------|-----------------------|
| `src/path/file.ts` | [What it does] | [How feature relates] |

### Existing Patterns

- **API:** [Pattern used]
- **Database:** [Pattern used]
- **Components:** [Pattern used]

### Reusable Code

- [Function/component có thể reuse]

### Dependencies & Conflicts

- Depends on: [X]
- Conflicts: [Y]

---

## 4. Implementation Plan

### Steps

- [ ] Step 1: [Description]
- [ ] Step 2: [Description]
- [ ] Step 3: [Description]

### Files to Change

| File           | Change        |
|----------------|---------------|
| `path/file.ts` | [Description] |

### Database Changes

```sql
-- N/A hoặc SQL changes
```

### API Changes

| Method | Endpoint | Description   |
|--------|----------|---------------|
| POST   | /api/xxx | [Description] |

---

## 5. Test Plan

### Test Cases

| ID    | Description  | Input   | Expected   | Priority |
|-------|--------------|---------|------------|----------|
| TC-01 | [Happy path] | [Input] | [Expected] | High     |
| TC-02 | [Edge case]  | [Input] | [Expected] | Medium   |
| TC-03 | [Error case] | [Input] | [Expected] | High     |

### Acceptance Criteria

- [ ] [Criteria 1]
- [ ] [Criteria 2]

---

## 6. Checkpoint

> **Fill khi Status = PAUSED**

**Paused at:** YYYY-MM-DD
**Reason:** [User request / New requirements / Blocker]
**Last step:** Step X

### Completed

- [x] Step 1 - Done
- [x] Step 2 - Done

### In Progress

- [ ] Step 3 - **Partial:** [What's done]

### Remaining

- [ ] Step 4
- [ ] Step 5

### New/Changed Requirements

- [NEW] [New requirement]
- [MODIFIED] [Changed requirement]

---

## 7. Implementation Summary

> **Fill khi Status = DONE** (để AI sessions sau đọc nhanh)

### What Changed

| File               | Change                    |
|--------------------|---------------------------|
| `src/path/file.ts` | [What was added/modified] |

### Key Decisions

| Decision   | Reason |
|------------|--------|
| [Decision] | [Why]  |

### Patterns Used

- [Pattern]: [Where applied]

### Quick Reference

- Entry point: `src/path/main.ts`
- Config: `setting_name` in settings table
- API: `POST /api/xxx`
