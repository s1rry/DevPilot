import ReactMarkdown from "react-markdown";
import rehypeHighlight from "rehype-highlight";
import remarkGfm from "remark-gfm";

import "highlight.js/styles/github-dark.css";

interface MarkdownProps {
  /** Markdown source to render. */
  content: string;
}

/**
 * Renders assistant messages as GitHub-flavored Markdown with syntax-
 * highlighted code blocks. Element styles are kept minimal and themed.
 */
export function Markdown({ content }: MarkdownProps) {
  return (
    <div className="markdown text-sm leading-relaxed text-fg">
      <ReactMarkdown
        remarkPlugins={[remarkGfm]}
        rehypePlugins={[rehypeHighlight]}
        components={{
          p: ({ children }) => <p className="my-2 first:mt-0 last:mb-0">{children}</p>,
          ul: ({ children }) => <ul className="my-2 list-disc pl-5">{children}</ul>,
          ol: ({ children }) => <ol className="my-2 list-decimal pl-5">{children}</ol>,
          li: ({ children }) => <li className="my-0.5">{children}</li>,
          a: ({ children, href }) => (
            <a href={href} className="text-accent underline">
              {children}
            </a>
          ),
          code: ({ className, children }) => {
            const isBlock = (className ?? "").includes("language-");
            if (isBlock) {
              return <code className={className}>{children}</code>;
            }
            return (
              <code className="rounded bg-elevated px-1 py-0.5 text-[0.85em]">{children}</code>
            );
          },
          pre: ({ children }) => (
            <pre className="my-2 overflow-auto rounded-md border border-border bg-surface p-3 text-xs">
              {children}
            </pre>
          ),
        }}
      >
        {content}
      </ReactMarkdown>
    </div>
  );
}
