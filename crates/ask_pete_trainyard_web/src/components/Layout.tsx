import { ReactNode } from 'react';

interface LayoutProps {
    children: ReactNode;
}

export default function Layout({ children }: LayoutProps) {
    return (
        <div className="app-layout">
            {/* Industrial Header */}
            <header className="app-header">
                <div className="flex items-center gap-4">
                    <div className="header-title">
                        <h1>THE TRAIN YARD</h1>
                        <div className="header-subtitle">Instructor Control Panel</div>
                    </div>
                </div>

                <nav className="header-nav">
                    <a href="#" className="nav-link">Graph Editor</a>
                    <a href="#" className="nav-link">Weigh Station</a>
                    <a href="#" className="nav-link">Knowledge Library</a>
                </nav>
            </header>

            {/* Main Content */}
            <main className="canvas-container">
                {children}
            </main>
        </div>
    );
}
