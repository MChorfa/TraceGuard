import React, { useState, useEffect } from 'react';
import { Tooltip } from 'antd';
import { useAuth } from '../../hooks/useAuth';

interface OnboardingTooltipProps {
  children: React.ReactNode;
  title: string;
  placement?: 'top' | 'left' | 'right' | 'bottom';
  tooltipId: string;
}

const OnboardingTooltip: React.FC<OnboardingTooltipProps> = ({
  children,
  title,
  placement = 'bottom',
  tooltipId,
}) => {
  const [visible, setVisible] = useState(false);
  const { user } = useAuth();

  useEffect(() => {
    if (user) {
      const tooltipSeen = localStorage.getItem(`tooltip_${tooltipId}_seen`);
      setVisible(!tooltipSeen);
    }
  }, [user, tooltipId]);

  const handleClose = () => {
    setVisible(false);
    localStorage.setItem(`tooltip_${tooltipId}_seen`, 'true');
  };

  return (
    <Tooltip
      title={
        <div>
          {title}
          <br />
          <a onClick={handleClose}>Got it!</a>
        </div>
      }
      visible={visible}
      placement={placement}
    >
      {children}
    </Tooltip>
  );
};

export default OnboardingTooltip;