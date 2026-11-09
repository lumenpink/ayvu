#!/bin/bash
set -eo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
README_FILE="${ROOT_DIR}/README.md"
MAX_LINES=50

echo "=== [TEST] Auditoria de teto de linhas de README.md (GOST-20) ==="

audit_readme_lines() {
    local file="$1"
    local count
    count=$(wc -l < "$file")
    if [ "$count" -gt "$MAX_LINES" ]; then
        echo "  [FAIL] Linhas ($count) excedem o teto de $MAX_LINES em $(basename "$file")"
        return 1
    fi
    echo "  [PASS] $(basename "$file") possui $count linhas (<= $MAX_LINES)"
    return 0
}

FAILED=0
if ! audit_readme_lines "$README_FILE"; then
    FAILED=1
fi

if [ $FAILED -ne 0 ]; then
    echo "❌ [RED TEST] Auditoria falhou: README.md excede o teto de $MAX_LINES linhas."
    exit 1
fi

echo "=== [MUTATION] Testes de Mutação (Sensibilidade ao teto de linhas) ==="
TMP_MUTANT=$(mktemp --suffix=.md)
trap 'rm -f "$TMP_MUTANT"' EXIT

for i in $(seq 1 55); do
    echo "Linha $i de conteúdo mutante" >> "$TMP_MUTANT"
done

if audit_readme_lines "$TMP_MUTANT" >/dev/null 2>&1; then
    echo "  [FAIL] Mutante com 55 linhas não foi detectado!"
    exit 1
else
    echo "  [PASS] Mutante com 55 linhas detectado com sucesso."
fi

echo "✅ Todos os testes de conformidade do README passaram."
